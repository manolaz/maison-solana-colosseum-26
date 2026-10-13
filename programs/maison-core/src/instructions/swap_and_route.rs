use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::constants::{PROTOCOL_CONFIG_SEED, ROUTE_PROFILE_SEED};
use crate::errors::MaisonError;
use crate::events::SwapAndRouteExecuted;
use crate::state::protocol_config::ProtocolConfig;
use crate::state::route_profile::RouteProfile;
use crate::utils::token::{calculate_protocol_fee, calculate_split_share, transfer_tokens_from_user};

#[derive(Accounts)]
pub struct SwapAndRoute<'info> {
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

    /// Input token mint (e.g. USDC or SOL)
    pub input_mint: InterfaceAccount<'info, Mint>,

    /// Output token mint (e.g. Philippine Peso stablecoin, Brazilian Real stablecoin, or SOL)
    pub output_mint: InterfaceAccount<'info, Mint>,

    /// Sender's input token source
    #[account(
        mut,
        token::mint = input_mint,
        token::authority = sender,
        token::token_program = token_program
    )]
    pub sender_input_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Protocol fee treasury token account for input tokens
    #[account(
        mut,
        token::mint = input_mint,
        token::authority = protocol_config.fee_treasury,
        token::token_program = token_program
    )]
    pub fee_treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Temporary intermediate vault or swap recipient for output tokens
    #[account(
        mut,
        token::mint = output_mint,
        token::authority = sender,
        token::token_program = token_program
    )]
    pub intermediate_output_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: DEX program interface (e.g. Jupiter / Whirlpool / Raydium router)
    pub dex_program: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handle_swap_and_route<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapAndRoute<'info>>,
    input_amount: u64,
    min_output_amount: u64,
    dex_instruction_data: Vec<u8>,
) -> Result<()> {
    require!(input_amount > 0, MaisonError::ZeroAmount);

    let clock = Clock::get()?;
    let fee_bps = ctx.accounts.protocol_config.protocol_fee_bps;
    let (net_input_amount, protocol_fee) = calculate_protocol_fee(input_amount, fee_bps)?;

    // 1. Deduct protocol fee in input tokens
    if protocol_fee > 0 {
        transfer_tokens_from_user(
            &ctx.accounts.token_program,
            &ctx.accounts.sender_input_token_account,
            &ctx.accounts.fee_treasury_token_account,
            &ctx.accounts.input_mint,
            &ctx.accounts.sender,
            protocol_fee,
        )?;
    }

    // 2. Execute DEX Swap CPI if instruction data is provided
    let balance_before = ctx.accounts.intermediate_output_account.amount;

    if !dex_instruction_data.is_empty() {
        // Construct arbitrary DEX CPI call using caller-provided accounts
        let account_metas: Vec<AccountMeta> = ctx
            .remaining_accounts
            .iter()
            .map(|acc| {
                if acc.is_writable {
                    AccountMeta::new(acc.key(), acc.is_signer)
                } else {
                    AccountMeta::new_readonly(acc.key(), acc.is_signer)
                }
            })
            .collect();

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.dex_program.key(),
            accounts: account_metas,
            data: dex_instruction_data,
        };

        anchor_lang::solana_program::program::invoke(&ix, ctx.remaining_accounts)?;
    }

    // Reload account to observe post-swap output balance
    ctx.accounts.intermediate_output_account.reload()?;
    let balance_after = ctx.accounts.intermediate_output_account.amount;
    let swapped_output = balance_after.saturating_sub(balance_before);

    // Slippage safety assertion
    require!(
        swapped_output >= min_output_amount || dex_instruction_data.is_empty(),
        MaisonError::SlippageExceeded
    );

    let amount_to_route = if swapped_output > 0 {
        swapped_output
    } else {
        min_output_amount
    };

    // 3. Update route spending tracker
    let route_profile = &mut ctx.accounts.route_profile;
    route_profile.track_spending(net_input_amount, clock.unix_timestamp)?;

    emit!(SwapAndRouteExecuted {
        sender: ctx.accounts.sender.key(),
        input_mint: ctx.accounts.input_mint.key(),
        output_mint: ctx.accounts.output_mint.key(),
        input_amount,
        output_amount: amount_to_route,
        fee_deducted: protocol_fee,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
