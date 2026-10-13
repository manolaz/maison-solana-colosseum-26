use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked};
use crate::errors::MaisonError;

/// Performs a token transfer using modern TokenInterface, compatible with both
/// legacy SPL Token and Token-2022 (Token Extensions), including mints with transfer hooks.
pub fn transfer_tokens_from_user<'info>(
    token_program: &Interface<'info, TokenInterface>,
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    transfer_checked(cpi_ctx, amount, mint.decimals)
}

/// Performs a token transfer signed by a PDA authority (e.g. escrow vault or protocol treasury).
pub fn transfer_tokens_from_pda<'info>(
    token_program: &Interface<'info, TokenInterface>,
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    authority_account: &AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority_account.clone(),
    };

    let cpi_ctx = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer_seeds);
    transfer_checked(cpi_ctx, amount, mint.decimals)
}

/// Computes the net remittance amount and protocol fee in basis points.
/// Returns `(net_amount, protocol_fee)`.
pub fn calculate_protocol_fee(
    gross_amount: u64,
    fee_bps: u16,
) -> Result<(u64, u64)> {
    if fee_bps == 0 || gross_amount == 0 {
        return Ok((gross_amount, 0));
    }

    let fee = (gross_amount as u128)
        .checked_mul(fee_bps as u128)
        .ok_or(MaisonError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(MaisonError::MathOverflow)? as u64;

    let net = gross_amount
        .checked_sub(fee)
        .ok_or(MaisonError::MathOverflow)?;

    Ok((net, fee))
}

/// Computes the split share for a recipient based on their allocation basis points.
pub fn calculate_split_share(
    total_amount: u64,
    basis_points: u16,
) -> Result<u64> {
    let share = (total_amount as u128)
        .checked_mul(basis_points as u128)
        .ok_or(MaisonError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(MaisonError::MathOverflow)? as u64;

    Ok(share)
}
