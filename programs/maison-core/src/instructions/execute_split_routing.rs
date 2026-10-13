use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::constants::{PROTOCOL_CONFIG_SEED, ROUTE_PROFILE_SEED};
use crate::errors::MaisonError;
use crate::events::SplitRoutingExecuted;
use crate::state::protocol_config::ProtocolConfig;
use crate::state::route_profile::RouteProfile;
use crate::utils::token::{calculate_protocol_fee, calculate_split_share, transfer_tokens_from_user};

#[derive(Accounts)]
pub struct ExecuteSplitRouting<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        constraint = !protocol_config.is_paused @ MaisonError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(
        mut,
        seeds = [ROUTE_PROFILE_SEED, sender.key().as_ref(), route_profile.profile_id.as_ref()],
        bump = route_profile.bump,
        constraint = route_profile.is_active @ MaisonError::RouteProfileInactive
    )]
    pub route_profile: Account<'info, RouteProfile>,

    /// Remittance token mint (supports standard SPL and Token-2022)
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// Sender's funding token account
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = sender,
        token::token_program = token_program
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Protocol fee treasury token account
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = protocol_config.fee_treasury,
        token::token_program = token_program
    )]
    pub fee_treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handle_execute_split_routing<'info>(
    ctx: Context<'_, '_, '_, 'info, ExecuteSplitRouting<'info>>,
    gross_amount: u64,
    external_reference: [u8; 32],
) -> Result<()> {
    require!(gross_amount > 0, MaisonError::ZeroAmount);
    require!(
        external_reference != [0u8; 32],
        MaisonError::InvalidExternalReference
    );

    let clock = Clock::get()?;
    let fee_bps = ctx.accounts.protocol_config.protocol_fee_bps;
    let (net_amount, protocol_fee) = calculate_protocol_fee(gross_amount, fee_bps)?;

    // 1. Deduct and transfer protocol fee to treasury
    if protocol_fee > 0 {
        transfer_tokens_from_user(
            &ctx.accounts.token_program,
            &ctx.accounts.sender_token_account,
            &ctx.accounts.fee_treasury_token_account,
            &ctx.accounts.token_mint,
            &ctx.accounts.sender,
            protocol_fee,
        )?;
    }

    // 2. Track spending limits on the profile
    let route_profile = &mut ctx.accounts.route_profile;
    route_profile.track_spending(net_amount, clock.unix_timestamp)?;

    let splits = route_profile.splits.clone();
    let num_splits = splits.len();

    // Verify remaining accounts match split count
    require!(
        ctx.remaining_accounts.len() >= num_splits,
        MaisonError::TooManySplitRecipients
    );

    // 3. Atomically distribute split shares to recipient token accounts
    let mut total_distributed: u64 = 0;
    for (i, split) in splits.iter().enumerate() {
        let recipient_acc_info = &ctx.remaining_accounts[i];

        // Calculate proportional share
        let mut share = calculate_split_share(net_amount, split.basis_points)?;

        // Give any rounding remainder to the final recipient to ensure 100% exact disbursement
        if i == num_splits - 1 {
            let expected_remainder = net_amount
                .checked_sub(total_distributed)
                .ok_or(MaisonError::MathOverflow)?;
            share = expected_remainder;
        }

        if share > 0 {
            // Load and validate recipient token account
            let recipient_token_account = InterfaceAccount::<TokenAccount>::try_from(recipient_acc_info)?;
            require!(
                recipient_token_account.mint == ctx.accounts.token_mint.key(),
                MaisonError::InvalidTokenMint
            );

            // Execute transfer
            let cpi_accounts = anchor_spl::token_interface::TransferChecked {
                from: ctx.accounts.sender_token_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                to: recipient_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            anchor_spl::token_interface::transfer_checked(cpi_ctx, share, ctx.accounts.token_mint.decimals)?;

            total_distributed = total_distributed
                .checked_add(share)
                .ok_or(MaisonError::MathOverflow)?;
        }
    }

    emit!(SplitRoutingExecuted {
        route_profile: Some(ctx.accounts.route_profile.key()),
        sender: ctx.accounts.sender.key(),
        token_mint: ctx.accounts.token_mint.key(),
        total_amount: gross_amount,
        protocol_fee,
        splits,
        external_reference,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
