use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub fee_bps: u16,
    pub bump: u8,
    pub authority_bump: u8,
}

impl Pool {
    // Each Pubkey is 32 bytes, the total size of the pool is 196
    pub const LEN: usize = 32 * 6 + 2 + 1 + 1;
}
