use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::remittance_escrow::ClaimMethod;
use state::route_profile::SplitAllocation;

declare_id!("MsnCore111111111111111111111111111111111111");

#[program]
pub mod maison_core {
    use super::*;

    /// Initialize global protocol state, treasury wallet, and fee basis points.
    pub fn initialize_protocol(
        ctx: Context<InitializeProtocol>,
        protocol_fee_bps: u16,
    ) -> Result<()> {
        handle_initialize_protocol(ctx, protocol_fee_bps)
    }

    /// Update protocol parameters (fee, treasury, emergency pause).
    pub fn update_protocol_config(
        ctx: Context<UpdateProtocolConfig>,
        new_fee_bps: Option<u16>,
        new_paused_state: Option<bool>,
    ) -> Result<()> {
        handle_update_protocol_config(ctx, new_fee_bps, new_paused_state)
    }

    /// Nominate a new protocol admin (Step 1 of 2-step transfer).
    pub fn transfer_admin(ctx: Context<TransferAdmin>) -> Result<()> {
        handle_transfer_admin(ctx)
    }

    /// Accept admin nomination (Step 2 of 2-step transfer).
    pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
        handle_accept_admin(ctx)
    }

    /// Create a personalized remittance route profile with multi-destination split rules.
    pub fn create_route_profile(
        ctx: Context<CreateRouteProfile>,
        profile_id: [u8; 32],
        splits: Vec<SplitAllocation>,
        daily_limit: u64,
        compliance_tier: u8,
    ) -> Result<()> {
        handle_create_route_profile(ctx, profile_id, splits, daily_limit, compliance_tier)
    }

    /// Update split allocations, daily spending limit, or active status.
    pub fn update_route_profile(
        ctx: Context<UpdateRouteProfile>,
        new_splits: Option<Vec<SplitAllocation>>,
        new_daily_limit: Option<u64>,
        new_is_active: Option<bool>,
    ) -> Result<()> {
        handle_update_route_profile(ctx, new_splits, new_daily_limit, new_is_active)
    }

    /// Deposit funds into a timelocked remittance escrow with conditional claim authorization.
    pub fn initiate_remittance(
        ctx: Context<InitiateRemittance>,
        external_reference: [u8; 32],
        gross_amount: u64,
        claim_method: ClaimMethod,
        claim_commitment: [u8; 32],
        timelock_duration_seconds: i64,
    ) -> Result<()> {
        handle_initiate_remittance(
            ctx,
            external_reference,
            gross_amount,
            claim_method,
            claim_commitment,
            timelock_duration_seconds,
        )
    }

    /// Atomically disburse an incoming remittance across configured split accounts.
    pub fn execute_split_routing<'info>(
        ctx: Context<'_, '_, '_, 'info, ExecuteSplitRouting<'info>>,
        gross_amount: u64,
        external_reference: [u8; 32],
    ) -> Result<()> {
        handle_execute_split_routing(ctx, gross_amount, external_reference)
    }

    /// Claim escrowed funds via Direct signature, Preimage hash, or WebAuthn/Passkey proof.
    pub fn claim_escrow(
        ctx: Context<ClaimEscrow>,
        claim_payload: Option<Vec<u8>>,
        authenticator_data: Option<Vec<u8>>,
        client_data_json: Option<Vec<u8>>,
    ) -> Result<()> {
        handle_claim_escrow(ctx, claim_payload, authenticator_data, client_data_json)
    }

    /// Reclaim expired escrowed funds back to the sender.
    pub fn refund_escrow(ctx: Context<RefundEscrow>) -> Result<()> {
        handle_refund_escrow(ctx)
    }

    /// Record and disburse incoming cross-chain remittance (Base CCTP / Tempo TIP-403).
    pub fn settle_cross_chain(
        ctx: Context<SettleCrossChain>,
        source_domain: u32,
        remote_nonce: u64,
        remote_sender: [u8; 32],
        amount: u64,
        external_reference: [u8; 32],
        compliance_verified: bool,
    ) -> Result<()> {
        handle_settle_cross_chain(
            ctx,
            source_domain,
            remote_nonce,
            remote_sender,
            amount,
            external_reference,
            compliance_verified,
        )
    }

    /// Swap input token via DEX CPI and route outputs according to route profile.
    pub fn swap_and_route<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapAndRoute<'info>>,
        input_amount: u64,
        min_output_amount: u64,
        dex_instruction_data: Vec<u8>,
    ) -> Result<()> {
        handle_swap_and_route(ctx, input_amount, min_output_amount, dex_instruction_data)
    }
}
