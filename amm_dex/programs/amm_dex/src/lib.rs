use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

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
}


