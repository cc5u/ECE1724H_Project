use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, Token, TokenAccount,
};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Pool::LEN,
        seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    // This check has to be added for safety issue IDK why
    /// CHECK: PDA authority for vaults and LP mint.
    #[account(
        seeds = [b"pool_authority", pool.key().as_ref()],
        bump
    )]
    pub pool_authority: UncheckedAccount<'info>,

    // Token mints (pre-existing SPL tokens)
    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    // Vault accounts that hold Pool's token A/B
    #[account(
        init,
        payer = payer,
        token::mint = token_a_mint,
        token::authority = pool_authority,
    )]
    pub token_a_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = token_b_mint,
        token::authority = pool_authority,
    )]
    pub token_b_vault: Account<'info, TokenAccount>,

    // LP token mint (for liquidity providers)
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,                
        mint::authority = pool_authority,
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_pool(
    ctx: Context<InitializePool>,
    fee_bps: u16,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.authority = ctx.accounts.pool_authority.key();
    pool.token_a_mint = ctx.accounts.token_a_mint.key();
    pool.token_b_mint = ctx.accounts.token_b_mint.key();
    pool.token_a_vault = ctx.accounts.token_a_vault.key();
    pool.token_b_vault = ctx.accounts.token_b_vault.key();
    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.fee_bps = fee_bps;
    pool.bump = ctx.bumps.pool;
    pool.authority_bump = ctx.bumps.pool_authority;

    Ok(())
}

