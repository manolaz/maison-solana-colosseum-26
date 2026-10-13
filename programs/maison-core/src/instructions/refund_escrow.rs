use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::constants::ESCROW_SEED;
use crate::errors::MaisonError;
use crate::events::EscrowRefunded;
use crate::state::remittance_escrow::{EscrowStatus, RemittanceEscrow};
use crate::utils::token::transfer_tokens_from_pda;

#[derive(Accounts)]
pub struct RefundEscrow<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, sender.key().as_ref(), escrow.external_reference.as_ref()],
        bump = escrow.bump,
        has_one = sender @ MaisonError::Unauthorized,
        constraint = escrow.status == EscrowStatus::Active @ MaisonError::EscrowAlreadyClaimed
    )]
    pub escrow: Account<'info, RemittanceEscrow>,

    #[account(
        address = escrow.token_mint @ MaisonError::InvalidTokenMint
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// Escrow token vault holding the remittance
    #[account(
        mut,
        address = escrow.escrow_vault,
        token::mint = token_mint,
        token::authority = escrow,
        token::token_program = token_program
    )]
    pub escrow_vault: InterfaceAccount<'info, TokenAccount>,

    /// Sender's token account receiving the refund
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = sender,
        token::token_program = token_program
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handle_refund_escrow(
    ctx: Context<RefundEscrow>,
) -> Result<()> {
    let clock = Clock::get()?;
    let escrow = &ctx.accounts.escrow;

    // Must be expired to refund
    require!(
        clock.unix_timestamp >= escrow.expiry_timestamp,
        MaisonError::EscrowNotExpired
    );

    let net_amount = escrow.net_amount;
    let sender_key = escrow.sender;
    let external_reference = escrow.external_reference;
    let bump = escrow.bump;

    let signer_seeds: &[&[&[u8]]] = &[&[
        ESCROW_SEED,
        sender_key.as_ref(),
        external_reference.as_ref(),
        &[bump],
    ]];

    // Refund tokens to sender
    transfer_tokens_from_pda(
        &ctx.accounts.token_program,
        &ctx.accounts.escrow_vault,
        &ctx.accounts.sender_token_account,
        &ctx.accounts.token_mint,
        &ctx.accounts.escrow.to_account_info(),
        signer_seeds,
        net_amount,
    )?;

    // Update status
    let escrow_mut = &mut ctx.accounts.escrow;
    escrow_mut.status = EscrowStatus::Refunded;

    emit!(EscrowRefunded {
        escrow: escrow_mut.key(),
        sender: sender_key,
        amount: net_amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
