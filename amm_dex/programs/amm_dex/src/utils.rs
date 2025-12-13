use crate::error::AmmError;

use anchor_lang::prelude::*;

const FEE_BASIS_POINTS_DENOM: u64 = 10_000; // 10000 = 100%

pub fn checked_mul(a: u64, b: u64) -> Result<u64> {
    a.checked_mul(b).ok_or(AmmError::MathOverflow.into())
}

pub fn checked_div(a: u64, b: u64) -> Result<u64> {
    a.checked_div(b).ok_or(AmmError::MathOverflow.into())
}

pub fn get_amount_out(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_bps: u16,
) -> Result<u64> {
    // Graceful message when the pool has no liquidity yet
    require!(
        reserve_in > 0 && reserve_out > 0,
        AmmError::InsufficientLiquidity
    );

    // Use u128 during math to avoid overflow on large trades against small pools.
    let fee_bps_u128 = fee_bps as u128;
    let amount_in_u128 = amount_in as u128;
    let reserve_in_u128 = reserve_in as u128;
    let reserve_out_u128 = reserve_out as u128;

    // amount_in_after_fee = amount_in * (1 - fee) / 10_000
    let amount_in_after_fee = amount_in_u128
        .checked_mul((FEE_BASIS_POINTS_DENOM as u128).saturating_sub(fee_bps_u128))
        .and_then(|n| n.checked_div(FEE_BASIS_POINTS_DENOM as u128))
        .ok_or(AmmError::MathOverflow)?;

    // constant product: amount_out = (amount_in_after_fee * reserve_out) / (reserve_in + amount_in_after_fee)
    let num = amount_in_after_fee
        .checked_mul(reserve_out_u128)
        .ok_or(AmmError::MathOverflow)?;
    let denom = reserve_in_u128
        .checked_add(amount_in_after_fee)
        .ok_or(AmmError::MathOverflow)?;

    let amount_out_u128 = num.checked_div(denom).ok_or(AmmError::MathOverflow)?;

    // If the trade is too large relative to pool size, return a friendly error.
    require!(amount_out_u128 > 0 && amount_out_u128 <= reserve_out_u128, AmmError::InsufficientLiquidity);

    u64::try_from(amount_out_u128).map_err(|_| AmmError::MathOverflow.into())
}
