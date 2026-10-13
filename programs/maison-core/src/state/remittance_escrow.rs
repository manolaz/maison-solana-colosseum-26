use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClaimMethod {
    /// Direct on-chain signature by the designated beneficiary
    DirectSignature,
    /// Verification of SHA256 / Keccak256 preimage voucher or secret claim code
    PreimageHash,
    /// WebAuthn / Passkey signature or attestation (e.g. for Tempo / biometric claim)
    WebAuthnPasskey,
    /// Relayer / Oracle attestation upon verified mobile money or fiat bank payout
    RelayerAttestation,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EscrowStatus {
    /// Funds locked in escrow, waiting for claim or expiry
    Active,
    /// Successfully released and disbursed to beneficiary
    Claimed,
    /// Timelock expired and funds refunded back to sender
    Refunded,
    /// Cancelled by authorized emergency governance
    Cancelled,
}

#[account]
pub struct RemittanceEscrow {
    /// Remittance originator
    pub sender: Pubkey,
    /// Intended beneficiary recipient
    pub beneficiary: Pubkey,
    /// Token mint being remitted (supports standard SPL and Token-2022)
    pub token_mint: Pubkey,
    /// SPL Token or Token-2022 program ID
    pub token_program: Pubkey,
    /// Escrow token vault holding the funds
    pub escrow_vault: Pubkey,
    /// Initial gross amount deposited by sender
    pub gross_amount: u64,
    /// Protocol fee deducted and transferred to treasury
    pub protocol_fee: u64,
    /// Net amount held in escrow for beneficiary
    pub net_amount: u64,
    /// Verification method required to claim
    pub claim_method: ClaimMethod,
    /// 32-byte cryptographic commitment or hash (e.g. sha256(secret) or passkey credential commitment)
    pub claim_commitment: [u8; 32],
    /// Creation Unix timestamp
    pub created_at: i64,
    /// Expiration Unix timestamp; sender can refund if now >= expiry_timestamp
    pub expiry_timestamp: i64,
    /// External banking / mobile money reference ID (32 bytes)
    pub external_reference: [u8; 32],
    /// Current lifecycle state
    pub status: EscrowStatus,
    /// Escrow state PDA bump
    pub bump: u8,
    /// Escrow token vault PDA bump
    pub vault_bump: u8,
    /// Reserved space for future extensions
    pub _reserved: [u8; 32],
}

impl RemittanceEscrow {
    pub const LEN: usize = 8 // discriminator
        + 32 // sender
        + 32 // beneficiary
        + 32 // token_mint
        + 32 // token_program
        + 32 // escrow_vault
        + 8 // gross_amount
        + 8 // protocol_fee
        + 8 // net_amount
        + 1 // claim_method
        + 32 // claim_commitment
        + 8 // created_at
        + 8 // expiry_timestamp
        + 32 // external_reference
        + 1 // status
        + 1 // bump
        + 1 // vault_bump
        + 32; // reserved
}
