use anchor_lang::prelude::*;
use instructions::*;
pub mod state;
pub mod instructions;
pub mod error;
pub mod utils;

declare_id!("Ag4VtauT33Q54JioCeAc29ZzbEFAqEfbCPTDq1jjvU2C");

#[program]
pub mod amm_dex {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_pool(ctx, fee_bps)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a_desired: u64,
        amount_b_desired: u64,
    ) -> Result<()> {
        add_liquidity::add_liquidity(ctx, amount_a_desired, amount_b_desired)
    }
}


