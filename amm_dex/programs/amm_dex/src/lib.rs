use anchor_lang::prelude::*;
use instructions::*;
pub mod state;
pub mod instructions;

declare_id!("2Cmtsw7yaqWoaya4Mxc9zXa1GyXWUVevwg2ZXr7oTDxV");

#[program]
pub mod amm_dex {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_pool(ctx, fee_bps)
    }
}


