use anchor_lang::prelude::*;

declare_id!("B1mAEmLXkPVTS1pqUUYUoWL1ATVGBaN7UDw6NVokh4Au");

#[program]
pub mod amm_dex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
