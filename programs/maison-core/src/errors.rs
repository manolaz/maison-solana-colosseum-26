use anchor_lang::prelude::*;

#[error_code]
pub enum MaisonError {
    #[msg("Protocol operations are currently paused by governance")]
    ProtocolPaused,

    #[msg("Caller is not authorized to execute this instruction")]
    Unauthorized,

    #[msg("Protocol fee basis points exceed the hard safety cap of 250 bps")]
    FeeBpsTooHigh,

    #[msg("Total split basis points must sum exactly to 10,000 bps (100%)")]
    InvalidSplitBpsTotal,

    #[msg("Number of split recipients exceeds the protocol limit of 10")]
    TooManySplitRecipients,

    #[msg("Route profile must have at least one valid recipient allocation")]
    EmptySplitAllocations,

    #[msg("Duplicate recipient detected in split routing configuration")]
    DuplicateSplitRecipient,

    #[msg("Zero amount cannot be remitted")]
    ZeroAmount,

    #[msg("Remittance exceeds route daily spending limit")]
    DailyLimitExceeded,

    #[msg("Account does not satisfy required KYC / TIP-403 compliance tier")]
    ComplianceTierMismatch,

    #[msg("Route profile is inactive")]
    RouteProfileInactive,

    #[msg("Escrow has already been claimed or released")]
    EscrowAlreadyClaimed,

    #[msg("Escrow has expired and can no longer be claimed by recipient")]
    EscrowExpired,

    #[msg("Escrow timelock has not expired yet; sender cannot refund")]
    EscrowNotExpired,

    #[msg("Provided claim preimage does not match stored claim hash")]
    InvalidClaimPreimage,

    #[msg("Provided WebAuthn / Passkey proof or signature is invalid")]
    InvalidWebAuthnProof,

    #[msg("Remittance settlement record already finalized")]
    SettlementAlreadyFinalized,

    #[msg("Invalid cross-chain source domain or relayer signature")]
    InvalidCrossChainDomain,

    #[msg("DEX swap slippage exceeded the minimum output threshold")]
    SlippageExceeded,

    #[msg("Arithmetic overflow occurred during balance or fee computation")]
    MathOverflow,

    #[msg("Mismatched token mint between instruction and route configuration")]
    InvalidTokenMint,

    #[msg("Invalid vault authority PDA derivation")]
    InvalidVaultAuthority,

    #[msg("Transfer fee deduction calculation resulted in an invalid net remittance")]
    InvalidTransferFeeCalculation,

    #[msg("External reference ID cannot be empty")]
    InvalidExternalReference,

    #[msg("Timelock duration is outside protocol allowed bounds")]
    InvalidTimelockDuration,
}
