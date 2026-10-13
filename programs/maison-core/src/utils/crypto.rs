use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash as sha256_hash;
use anchor_lang::solana_program::keccak::hash as keccak_hash;
use crate::errors::MaisonError;

/// Verifies a secret claim preimage against the stored commitment.
/// Checks either standard SHA-256 or Keccak-256.
pub fn verify_claim_preimage(
    preimage: &[u8],
    expected_commitment: &[u8; 32],
) -> Result<()> {
    let sha_digest = sha256_hash(preimage).to_bytes();
    let keccak_digest = keccak_hash(preimage).0;

    if &sha_digest == expected_commitment || &keccak_digest == expected_commitment {
        Ok(())
    } else {
        err!(MaisonError::InvalidClaimPreimage)
    }
}

/// Verifies a WebAuthn / Passkey proof payload against the stored commitment.
/// In the WebAuthn standard (and Tempo TIP-403 WebAuthn flow), the credential
/// authenticator produces a clientDataJSON and authenticatorData signed over a challenge.
/// The commitment stores the cryptographic hash of the expected passkey credentials.
pub fn verify_webauthn_proof(
    authenticator_data: &[u8],
    client_data_json: &[u8],
    expected_commitment: &[u8; 32],
) -> Result<()> {
    require!(!authenticator_data.is_empty(), MaisonError::InvalidWebAuthnProof);
    require!(!client_data_json.is_empty(), MaisonError::InvalidWebAuthnProof);

    // Compute composite hash of authenticatorData + sha256(clientDataJSON)
    let client_data_hash = sha256_hash(client_data_json).to_bytes();
    let mut composite_input = Vec::with_capacity(authenticator_data.len() + 32);
    composite_input.extend_from_slice(authenticator_data);
    composite_input.extend_from_slice(&client_data_hash);

    let composite_hash = sha256_hash(&composite_input).to_bytes();

    // Verify composite hash matches expected commitment or direct commitment match
    if &composite_hash == expected_commitment || &client_data_hash == expected_commitment {
        Ok(())
    } else {
        err!(MaisonError::InvalidWebAuthnProof)
    }
}
