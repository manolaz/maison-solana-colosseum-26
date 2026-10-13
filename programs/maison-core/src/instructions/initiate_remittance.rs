use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::constants::{
    ESCROW_SEED, MAX_ESCROW_TIMELOCK_SECONDS, MIN_ESCROW_TIMELOCK_SECONDS, PROTOCOL_CONFIG_SEED,
};
use crate::errors::MaisonError;
use crate::events::RemittanceInitiated;
use crate::state::protocol_config::ProtocolConfig;
use crate::state::remittance_escrow::{ClaimMethod, EscrowStatus, RemittanceEscrow};
use crate::utils::token::{calculate_protocol_fee, transfer_tokens_from_user};

#[derive(Accounts)]
#[instruction(external_reference: [u8; 32])]
pub struct InitiateRemittance<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        constraint = !protocol_config.is_paused @ MaisonError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(
        init,
        payer = sender,
        space = RemittanceEscrow::LEN,
        seeds = [ESCROW_SEED, sender.key().as_ref(), external_reference.as_ref()],
        bump
    )]
    pub escrow: Account<'info, RemittanceEscrow>,

    /// Token mint being remitted (supports standard SPL and Token-2022 with transfer hooks)
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// Sender's token source account
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

    /// Token vault owned by the escrow PDA
    #[account(
        init,
        payer = sender,
        token::mint = token_mint,
        token::authority = escrow,
        token::token_program = token_program,
        seeds = [ESCROW_SEED, escrow.key().as_ref(), b"vault"],
        bump
    )]
    pub escrow_vault: InterfaceAccount<'info, TokenAccount>,

    /// Beneficiary who will claim or receive the payout
    /// CHECK: Beneficiary public key or recipient wallet
    pub beneficiary: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initiate_remittance(
    ctx: Context<InitiateRemittance>,
    external_reference: [u8; 32],
    gross_amount: u64,
    claim_method: ClaimMethod,
    claim_commitment: [u8; 32],
    timelock_duration_seconds: i64,
) -> Result<()> {
    require!(gross_amount > 0, MaisonError::ZeroAmount);
    require!(
        external_reference != [0u8; 32],
        MaisonError::InvalidExternalReference
    );
    require!(
        timelock_duration_seconds >= MIN_ESCROW_TIMELOCK_SECONDS
            && timelock_duration_seconds <= MAX_ESCROW_TIMELOCK_SECONDS,
        MaisonError::InvalidTimelockDuration
    );

    let clock = Clock::get()?;
    let expiry_timestamp = clock
        .unix_timestamp
        .checked_add(timelock_duration_seconds)
        .ok_or(MaisonError::MathOverflow)?;

    let fee_bps = ctx.accounts.protocol_config.protocol_fee_bps;
    let (net_amount, protocol_fee) = calculate_protocol_fee(gross_amount, fee_bps)?;

    // 1. Transfer protocol fee directly to treasury token account
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

    // 2. Transfer net remittance into escrow vault
    transfer_tokens_from_user(
        &ctx.accounts.token_program,
        &ctx.accounts.sender_token_account,
        &ctx.accounts.escrow_vault,
        &ctx.accounts.token_mint,
        &ctx.accounts.sender,
        net_amount,
    )?;

    // 3. Initialize escrow state
    let escrow = &mut ctx.accounts.escrow;
    escrow.sender = ctx.accounts.sender.key();
    escrow.beneficiary = ctx.accounts.beneficiary.key();
    escrow.token_mint = ctx.accounts.token_mint.key();
    escrow.token_program = ctx.accounts.token_program.key();
    escrow.escrow_vault = ctx.accounts.escrow_vault.key();
    escrow.gross_amount = gross_amount;
    escrow.protocol_fee = protocol_fee;
    escrow.net_amount = net_amount;
    escrow.claim_method = claim_method;
    escrow.claim_commitment = claim_commitment;
    escrow.created_at = clock.unix_timestamp;
    escrow.expiry_timestamp = expiry_timestamp;
    escrow.external_reference = external_reference;
    escrow.status = EscrowStatus::Active;
    escrow.bump = ctx.bumps.escrow;
    escrow.vault_bump = ctx.bumps.escrow_vault;
    escrow._reserved = [0u8; 32];

    emit!(RemittanceInitiated {
        escrow: escrow.key(),
        sender: escrow.sender,
        token_mint: escrow.token_mint,
        gross_amount,
        protocol_fee,
        net_amount,
        claim_method,
        expiry_timestamp,
        external_reference,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
