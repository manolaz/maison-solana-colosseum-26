use anchor_lang::prelude::*;

#[account]
pub struct SettlementRecord {
    /// Remote cross-chain source domain ID (e.g., 6 for Base CCTP, 100 for Tempo)
    pub source_domain: u32,
    /// Nonce emitted by the originating cross-chain message / CCTP burn
    pub remote_nonce: u64,
    /// Originating sender address on remote chain (32-byte padded EVM or Tempo address)
    pub remote_sender: [u8; 32],
    /// Target Solana recipient or remittance route profile
    pub recipient: Pubkey,
    /// Settled token mint (e.g., USDC, EURC, or local stablecoin)
    pub token_mint: Pubkey,
    /// Net settled amount
    pub amount: u64,
    /// External banking / mobile money remittance reference
    pub external_reference: [u8; 32],
    /// Whether TIP-403 / AML compliance attestation was verified
    pub compliance_verified: bool,
    /// Timestamp when settlement was finalized on Solana
    pub settled_at: i64,
    /// Authorized relayer or attestation keeper pubkey
    pub relayer: Pubkey,
    /// PDA bump
    pub bump: u8,
    /// Reserved space for future extensions
    pub _reserved: [u8; 32],
}

impl SettlementRecord {
    pub const LEN: usize = 8 // discriminator
        + 4 // source_domain
        + 8 // remote_nonce
        + 32 // remote_sender
        + 32 // recipient
        + 32 // token_mint
        + 8 // amount
        + 32 // external_reference
        + 1 // compliance_verified
        + 8 // settled_at
        + 32 // relayer
        + 1 // bump
        + 32; // reserved
}
