use anchor_lang::prelude::*;

#[account]
pub struct ProtocolConfig {
    /// Governance and super-admin authority
    pub admin: Pubkey,
    /// Pending admin for two-step authority handoff
    pub pending_admin: Option<Pubkey>,
    /// Protocol fee destination vault / wallet
    pub fee_treasury: Pubkey,
    /// Protocol fee in basis points (1 bps = 0.01%)
    pub protocol_fee_bps: u16,
    /// Emergency circuit breaker flag
    pub is_paused: bool,
    /// PDA bump seed
    pub bump: u8,
    /// Lifetime total remittance count processed
    pub total_remittances_count: u64,
    /// Lifetime cumulative volume (scaled in micro-units)
    pub total_volume_accumulated: u128,
    /// Reserved space for future extensions
    pub _reserved: [u8; 64],
}

impl ProtocolConfig {
    pub const LEN: usize = 8 // discriminator
        + 32 // admin
        + (1 + 32) // pending_admin
        + 32 // fee_treasury
        + 2 // protocol_fee_bps
        + 1 // is_paused
        + 1 // bump
        + 8 // total_remittances_count
        + 16 // total_volume_accumulated
        + 64; // reserved
}
