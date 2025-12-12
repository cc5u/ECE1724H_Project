use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Insufficient output amount for the requested swap.")]
    InsufficientOutputAmount,

    #[msg("Cannot add liquidity with zero LP supply and zero amounts.")]
    InvalidInitialLiquidity,

    #[msg("LP supply must be non-zero for this operation.")]
    InvalidLpSupply,

    #[msg("Math overflow occurred.")]
    MathOverflow,
}
