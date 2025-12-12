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
    require!(reserve_in > 0 && reserve_out > 0, AmmError::MathOverflow);

    let fee_bps_u64 = fee_bps as u64;
    let numerator = checked_mul(amount_in, FEE_BASIS_POINTS_DENOM - fee_bps_u64)?;
    let amount_in_after_fee = checked_div(numerator, FEE_BASIS_POINTS_DENOM)?;

    let num = checked_mul(amount_in_after_fee, reserve_out)?;
    let denom = reserve_in
        .checked_add(amount_in_after_fee)
        .ok_or(AmmError::MathOverflow)?;

    checked_div(num, denom)
}
