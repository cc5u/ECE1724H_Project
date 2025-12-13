use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub pool_id: u64,
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
    // 6 pubkeys (32 bytes each), 1 u64, 1 u16, 2 u8 => 204 bytes total (not counting the 8-byte discriminator)
    pub const LEN: usize = 8 + 32 * 6 + 2 + 1 + 1;
}

// PoolCounter allow the program automatically tracking the Pool ID
#[account]
pub struct PoolCounter {
    pub next_id: u64,
}

impl PoolCounter {
    pub const LEN: usize = 8;
}
