use anchor_lang::prelude::*;
use crate::constants::{MAX_SPLIT_RECIPIENTS, TOTAL_BPS};
use crate::errors::MaisonError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VaultType {
    /// Immediate disbursement directly to beneficiary token account
    Immediate,
    /// Timelocked savings vault
    SavingsEscrow,
    /// Dedicated medical/education emergency fund
    EmergencyEscrow,
    /// Mobile money / off-ramp escrow
    OffRampEscrow,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct SplitAllocation {
    /// Target beneficiary or sub-vault public key
    pub recipient: Pubkey,
    /// Share in basis points (10,000 bps = 100%)
    pub basis_points: u16,
    /// Destination categorization label (e.g. "Living Expenses", "Savings")
    pub label: [u8; 32],
    /// Vault type execution pattern
    pub vault_type: VaultType,
}

impl SplitAllocation {
    pub const LEN: usize = 32 // recipient
        + 2 // basis_points
        + 32 // label
        + 1; // vault_type enum
}

#[account]
pub struct RouteProfile {
    /// Creator and manager of this remittance route
    pub owner: Pubkey,
    /// Unique profile identifier (or hash)
    pub profile_id: [u8; 32],
    /// PDA bump
    pub bump: u8,
    /// Status flag
    pub is_active: bool,
    /// Regulatory & KYC tier (0: Tier0, 1: Tier1, 2: Tier2, 3: TIP-403 compliant)
    pub compliance_tier: u8,
    /// 24-hour rolling cap on total remittance volume (0 = unlimited)
    pub daily_limit: u64,
    /// Cumulative volume remitted in current epoch
    pub daily_spent: u64,
    /// Timestamp marking the start of current 24-hour epoch
    pub last_reset_timestamp: i64,
    /// Configured split allocations (sum must equal 10,000 bps)
    pub splits: Vec<SplitAllocation>,
    /// Reserved space for future extensions
    pub _reserved: [u8; 32],
}

impl RouteProfile {
    pub const BASE_LEN: usize = 8 // discriminator
        + 32 // owner
        + 32 // profile_id
        + 1 // bump
        + 1 // is_active
        + 1 // compliance_tier
        + 8 // daily_limit
        + 8 // daily_spent
        + 8 // last_reset_timestamp
        + 4 // vec prefix
        + (MAX_SPLIT_RECIPIENTS * SplitAllocation::LEN) // max elements
        + 32; // reserved

    /// Validates split allocations and uniqueness
    pub fn validate_splits(splits: &[SplitAllocation]) -> Result<()> {
        require!(!splits.is_empty(), MaisonError::EmptySplitAllocations);
        require!(
            splits.len() <= MAX_SPLIT_RECIPIENTS,
            MaisonError::TooManySplitRecipients
        );

        let mut total_bps: u32 = 0;
        for (i, split) in splits.iter().enumerate() {
            total_bps = total_bps
                .checked_add(split.basis_points as u32)
                .ok_or(MaisonError::MathOverflow)?;

            // Ensure no duplicate recipients
            for other in &splits[i + 1..] {
                require!(
                    split.recipient != other.recipient,
                    MaisonError::DuplicateSplitRecipient
                );
            }
        }

        require!(
            total_bps == TOTAL_BPS as u32,
            MaisonError::InvalidSplitBpsTotal
        );

        Ok(())
    }

    /// Checks daily spending limit and resets epoch if 24 hours have elapsed
    pub fn track_spending(&mut self, amount: u64, current_timestamp: i64) -> Result<()> {
        if self.daily_limit == 0 {
            return Ok(());
        }

        let seconds_in_day: i64 = 86_400;
        if current_timestamp.saturating_sub(self.last_reset_timestamp) >= seconds_in_day {
            self.daily_spent = 0;
            self.last_reset_timestamp = current_timestamp;
        }

        let new_spent = self
            .daily_spent
            .checked_add(amount)
            .ok_or(MaisonError::MathOverflow)?;

        require!(
            new_spent <= self.daily_limit,
            MaisonError::DailyLimitExceeded
        );

        self.daily_spent = new_spent;
        Ok(())
    }
}
