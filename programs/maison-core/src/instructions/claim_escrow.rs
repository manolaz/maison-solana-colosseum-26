use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::constants::ESCROW_SEED;
use crate::errors::MaisonError;
use crate::events::EscrowClaimed;
use crate::state::remittance_escrow::{ClaimMethod, EscrowStatus, RemittanceEscrow};
use crate::utils::crypto::{verify_claim_preimage, verify_webauthn_proof};
use crate::utils::token::transfer_tokens_from_pda;

#[derive(Accounts)]
pub struct ClaimEscrow<'info> {
    #[account(mut)]
    pub claimant: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, escrow.sender.as_ref(), escrow.external_reference.as_ref()],
        bump = escrow.bump,
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

    /// Claimant's receiving token account
    #[account(
        mut,
        token::mint = token_mint,
        token::token_program = token_program
    )]
    pub claimant_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handle_claim_escrow(
    ctx: Context<ClaimEscrow>,
    claim_payload: Option<Vec<u8>>,
    authenticator_data: Option<Vec<u8>>,
    client_data_json: Option<Vec<u8>>,
) -> Result<()> {
    let clock = Clock::get()?;
    let escrow = &ctx.accounts.escrow;

    // Verify not expired
    require!(
        clock.unix_timestamp < escrow.expiry_timestamp,
        MaisonError::EscrowExpired
    );

    // Verify claim authorization based on method
    match escrow.claim_method {
        ClaimMethod::DirectSignature => {
            require!(
                ctx.accounts.claimant.key() == escrow.beneficiary,
                MaisonError::Unauthorized
            );
        }
        ClaimMethod::PreimageHash => {
            let preimage = claim_payload.ok_or(MaisonError::InvalidClaimPreimage)?;
            verify_claim_preimage(&preimage, &escrow.claim_commitment)?;
        }
        ClaimMethod::WebAuthnPasskey => {
            let auth_data = authenticator_data.ok_or(MaisonError::InvalidWebAuthnProof)?;
            let client_data = client_data_json.ok_or(MaisonError::InvalidWebAuthnProof)?;
            verify_webauthn_proof(&auth_data, &client_data, &escrow.claim_commitment)?;
        }
        ClaimMethod::RelayerAttestation => {
            // Relayer or beneficiary can finalize
            require!(
                ctx.accounts.claimant.key() == escrow.beneficiary
                    || ctx.accounts.claimant.key() == escrow.sender,
                MaisonError::Unauthorized
            );
        }
    }

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

    // Transfer net amount from escrow vault to claimant
    transfer_tokens_from_pda(
        &ctx.accounts.token_program,
        &ctx.accounts.escrow_vault,
        &ctx.accounts.claimant_token_account,
        &ctx.accounts.token_mint,
        &ctx.accounts.escrow.to_account_info(),
        signer_seeds,
        net_amount,
    )?;

    // Update status
    let escrow_mut = &mut ctx.accounts.escrow;
    escrow_mut.status = EscrowStatus::Claimed;

    emit!(EscrowClaimed {
        escrow: escrow_mut.key(),
        claimant: ctx.accounts.claimant.key(),
        amount: net_amount,
        claim_method: escrow_mut.claim_method,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
