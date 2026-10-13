use anchor_lang::prelude::*;
use crate::constants::{MAX_PROTOCOL_FEE_BPS, PROTOCOL_CONFIG_SEED};
use crate::errors::MaisonError;
use crate::events::ProtocolInitialized;
use crate::state::protocol_config::ProtocolConfig;

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ProtocolConfig::LEN,
        seeds = [PROTOCOL_CONFIG_SEED],
        bump
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    /// CHECK: Protocol treasury wallet that collects fees
    pub fee_treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_protocol(
    ctx: Context<InitializeProtocol>,
    protocol_fee_bps: u16,
) -> Result<()> {
    require!(
        protocol_fee_bps <= MAX_PROTOCOL_FEE_BPS,
        MaisonError::FeeBpsTooHigh
    );

    let clock = Clock::get()?;
    let config = &mut ctx.accounts.protocol_config;

    config.admin = ctx.accounts.admin.key();
    config.pending_admin = None;
    config.fee_treasury = ctx.accounts.fee_treasury.key();
    config.protocol_fee_bps = protocol_fee_bps;
    config.is_paused = false;
    config.bump = ctx.bumps.protocol_config;
    config.total_remittances_count = 0;
    config.total_volume_accumulated = 0;
    config._reserved = [0u8; 64];

    emit!(ProtocolInitialized {
        admin: config.admin,
        fee_treasury: config.fee_treasury,
        protocol_fee_bps,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
