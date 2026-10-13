use anchor_lang::prelude::*;
use crate::state::route_profile::SplitAllocation;
use crate::state::remittance_escrow::ClaimMethod;

#[event]
pub struct ProtocolInitialized {
    pub admin: Pubkey,
    pub fee_treasury: Pubkey,
    pub protocol_fee_bps: u16,
    pub timestamp: i64,
}

#[event]
pub struct ProtocolConfigUpdated {
    pub admin: Pubkey,
    pub fee_treasury: Pubkey,
    pub protocol_fee_bps: u16,
    pub paused: bool,
    pub timestamp: i64,
}

#[event]
pub struct RouteProfileCreated {
    pub profile: Pubkey,
    pub owner: Pubkey,
    pub profile_id: [u8; 32],
    pub split_count: u8,
    pub daily_limit: u64,
    pub compliance_tier: u8,
    pub timestamp: i64,
}

#[event]
pub struct RouteProfileUpdated {
    pub profile: Pubkey,
    pub owner: Pubkey,
    pub split_count: u8,
    pub daily_limit: u64,
    pub is_active: bool,
    pub timestamp: i64,
}

#[event]
pub struct RemittanceInitiated {
    pub escrow: Pubkey,
    pub sender: Pubkey,
    pub token_mint: Pubkey,
    pub gross_amount: u64,
    pub protocol_fee: u64,
    pub net_amount: u64,
    pub claim_method: ClaimMethod,
    pub expiry_timestamp: i64,
    pub external_reference: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct SplitRoutingExecuted {
    pub route_profile: Option<Pubkey>,
    pub sender: Pubkey,
    pub token_mint: Pubkey,
    pub total_amount: u64,
    pub protocol_fee: u64,
    pub splits: Vec<SplitAllocation>,
    pub external_reference: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct EscrowClaimed {
    pub escrow: Pubkey,
    pub claimant: Pubkey,
    pub amount: u64,
    pub claim_method: ClaimMethod,
    pub timestamp: i64,
}

#[event]
pub struct EscrowRefunded {
    pub escrow: Pubkey,
    pub sender: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct CrossChainSettled {
    pub settlement_record: Pubkey,
    pub source_domain: u32,
    pub remote_nonce: u64,
    pub recipient: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub external_reference: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct SwapAndRouteExecuted {
    pub sender: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub fee_deducted: u64,
    pub timestamp: i64,
}
