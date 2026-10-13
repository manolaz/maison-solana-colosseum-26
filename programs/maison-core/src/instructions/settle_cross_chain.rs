use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::constants::{PROTOCOL_CONFIG_SEED, SETTLEMENT_SEED};
use crate::errors::MaisonError;
use crate::events::CrossChainSettled;
use crate::state::protocol_config::ProtocolConfig;
use crate::state::settlement_record::SettlementRecord;
use crate::utils::token::transfer_tokens_from_user;

#[derive(Accounts)]
#[instruction(source_domain: u32, remote_nonce: u64)]
pub struct SettleCrossChain<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        constraint = !protocol_config.is_paused @ MaisonError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    #[account(
        init,
        payer = relayer,
        space = SettlementRecord::LEN,
        seeds = [
            SETTLEMENT_SEED,
            source_domain.to_le_bytes().as_ref(),
            remote_nonce.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub settlement_record: Account<'info, SettlementRecord>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    /// Relayer or cross-chain bridge buffer token account
    #[account(
        mut,
        token::mint = token_mint,
        token::authority = relayer,
        token::token_program = token_program
    )]
    pub relayer_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Recipient's destination token account
    #[account(
        mut,
        token::mint = token_mint,
        token::token_program = token_program
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Target recipient account
    pub recipient: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_settle_cross_chain(
    ctx: Context<SettleCrossChain>,
    source_domain: u32,
    remote_nonce: u64,
    remote_sender: [u8; 32],
    amount: u64,
    external_reference: [u8; 32],
    compliance_verified: bool,
) -> Result<()> {
    require!(amount > 0, MaisonError::ZeroAmount);
    require!(
        external_reference != [0u8; 32],
        MaisonError::InvalidExternalReference
    );

    let clock = Clock::get()?;

    // Disburse settled cross-chain funds to recipient token account
    transfer_tokens_from_user(
        &ctx.accounts.token_program,
        &ctx.accounts.relayer_token_account,
        &ctx.accounts.recipient_token_account,
        &ctx.accounts.token_mint,
        &ctx.accounts.relayer,
        amount,
    )?;

    let record = &mut ctx.accounts.settlement_record;
    record.source_domain = source_domain;
    record.remote_nonce = remote_nonce;
    record.remote_sender = remote_sender;
    record.recipient = ctx.accounts.recipient.key();
    record.token_mint = ctx.accounts.token_mint.key();
    record.amount = amount;
    record.external_reference = external_reference;
    record.compliance_verified = compliance_verified;
    record.settled_at = clock.unix_timestamp;
    record.relayer = ctx.accounts.relayer.key();
    record.bump = ctx.bumps.settlement_record;
    record._reserved = [0u8; 32];

    emit!(CrossChainSettled {
        settlement_record: record.key(),
        source_domain,
        remote_nonce,
        recipient: record.recipient,
        token_mint: record.token_mint,
        amount,
        external_reference,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
