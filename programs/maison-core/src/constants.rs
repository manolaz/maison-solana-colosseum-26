//! Protocol-wide constants and PDA seed definitions for Maison Core.

/// Seed prefix for the global protocol configuration PDA.
pub const PROTOCOL_CONFIG_SEED: &[u8] = b"protocol_config";

/// Seed prefix for remittance route profile PDAs.
pub const ROUTE_PROFILE_SEED: &[u8] = b"route_profile";

/// Seed prefix for remittance escrow PDAs.
pub const ESCROW_SEED: &[u8] = b"escrow";

/// Seed prefix for cross-chain settlement record PDAs.
pub const SETTLEMENT_SEED: &[u8] = b"settlement";

/// Seed prefix for protocol vault authority.
pub const VAULT_AUTHORITY_SEED: &[u8] = b"vault_authority";

/// Maximum allowed split recipients per remittance route profile.
pub const MAX_SPLIT_RECIPIENTS: usize = 10;

/// Total basis points representing 100.00%.
pub const TOTAL_BPS: u16 = 10_000;

/// Maximum protocol fee cap (e.g., 250 bps = 2.50%).
pub const MAX_PROTOCOL_FEE_BPS: u16 = 250;

/// Minimum escrow timelock duration in seconds (10 minutes).
pub const MIN_ESCROW_TIMELOCK_SECONDS: i64 = 600;

/// Maximum escrow timelock duration in seconds (90 days).
pub const MAX_ESCROW_TIMELOCK_SECONDS: i64 = 90 * 86_400;

/// Standard length of string identifier / label hashes.
pub const HASH_LEN: usize = 32;

/// Standard length of external remittance reference identifier.
pub const EXTERNAL_REF_LEN: usize = 32;
