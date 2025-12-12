use anchor_lang::prelude::*;
use instructions::*;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("2Cmtsw7yaqWoaya4Mxc9zXa1GyXWUVevwg2ZXr7oTDxV");

#[program]
pub mod amm_dex {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
        instructions::initialize_pool(ctx, fee_bps)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a_desired: u64,
        amount_b_desired: u64,
    ) -> Result<()> {
        add_liquidity::add_liquidity(ctx, amount_a_desired, amount_b_desired)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
        rm_liquidity::remove_liquidity(ctx, lp_amount)
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_out: u64, is_a_to_b: bool) -> Result<()> {
        swap::swap(ctx, amount_in, minimum_out, is_a_to_b)
    }
}
