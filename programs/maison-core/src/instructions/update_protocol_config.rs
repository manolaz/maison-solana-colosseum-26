use anchor_lang::prelude::*;
use crate::constants::{MAX_PROTOCOL_FEE_BPS, PROTOCOL_CONFIG_SEED};
use crate::errors::MaisonError;
use crate::events::ProtocolConfigUpdated;
use crate::state::protocol_config::ProtocolConfig;

#[derive(Accounts)]
pub struct UpdateProtocolConfig<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        has_one = admin @ MaisonError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    /// CHECK: Optional new fee treasury account
    pub new_fee_treasury: Option<AccountInfo<'info>>,
}

#[derive(Accounts)]
pub struct TransferAdmin<'info> {
    pub current_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        has_one = admin @ MaisonError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    /// CHECK: Nominated new admin
    pub new_admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AcceptAdmin<'info> {
    pub pending_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_CONFIG_SEED],
        bump = protocol_config.bump,
        constraint = protocol_config.pending_admin == Some(pending_admin.key()) @ MaisonError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
}

pub fn handle_update_protocol_config(
    ctx: Context<UpdateProtocolConfig>,
    new_fee_bps: Option<u16>,
    new_paused_state: Option<bool>,
) -> Result<()> {
    let clock = Clock::get()?;
    let config = &mut ctx.accounts.protocol_config;

    if let Some(fee_bps) = new_fee_bps {
        require!(fee_bps <= MAX_PROTOCOL_FEE_BPS, MaisonError::FeeBpsTooHigh);
        config.protocol_fee_bps = fee_bps;
    }

    if let Some(treasury) = &ctx.accounts.new_fee_treasury {
        config.fee_treasury = treasury.key();
    }

    if let Some(paused) = new_paused_state {
        config.is_paused = paused;
    }

    emit!(ProtocolConfigUpdated {
        admin: config.admin,
        fee_treasury: config.fee_treasury,
        protocol_fee_bps: config.protocol_fee_bps,
        paused: config.is_paused,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

pub fn handle_transfer_admin(
    ctx: Context<TransferAdmin>,
) -> Result<()> {
    let config = &mut ctx.accounts.protocol_config;
    config.pending_admin = Some(ctx.accounts.new_admin.key());
    Ok(())
}

pub fn handle_accept_admin(
    ctx: Context<AcceptAdmin>,
) -> Result<()> {
    let config = &mut ctx.accounts.protocol_config;
    config.admin = ctx.accounts.pending_admin.key();
    config.pending_admin = None;
    Ok(())
}
