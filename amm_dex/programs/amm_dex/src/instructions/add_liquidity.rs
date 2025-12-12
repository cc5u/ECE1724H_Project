use crate::state::*;
use crate::error::AmmError;
use crate::utils::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, Token, TokenAccount, Transfer, MintTo,
};

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// CHECK: PDA authority
    #[account(
        seeds = [b"pool_authority", pool.key().as_ref()],
        bump = pool.bump,
    )]
    pub pool_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_a_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b_vault: Account<'info, TokenAccount>,

    // User token accounts (must match mints in pool)
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,

    // LP mint and user LP token account
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_lp_token.mint == lp_mint.key(),
        constraint = user_lp_token.owner == user.key(),
    )]
    pub user_lp_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn add_liquidity(
    ctx: Context<AddLiquidity>,
    amount_a_desired: u64,
    amount_b_desired: u64,
) -> Result<()> {
    // 1. Compute actual amounts based on current pool ratio (or accept desired amounts for first LP).
    // 2. Transfer tokens from user -> vaults (token::transfer).
    // 3. Mint LP tokens based on contribution share (token::mint_to).

    let pool = &ctx.accounts.pool;

    let token_a_vault = &ctx.accounts.token_a_vault;
    let token_b_vault = &ctx.accounts.token_b_vault;
    let lp_mint = &ctx.accounts.lp_mint;

    let user_token_a = &ctx.accounts.user_token_a;
    let user_token_b = &ctx.accounts.user_token_b;
    let user_lp_token = &ctx.accounts.user_lp_token;

    let token_program = &ctx.accounts.token_program;

    let reserve_a = token_a_vault.amount;
    let reserve_b = token_b_vault.amount;
    let total_lp_supply = lp_mint.supply;

    // Determine how much A and B to actually take from the user
    let (amount_a, amount_b, lp_to_mint) = if total_lp_supply == 0 {
        // Initial liquidity provider
        require!(
            amount_a_desired > 0 && amount_b_desired > 0,
            AmmError::InvalidInitialLiquidity
        );

        // Very simple LP valuation: lp = amount_a + amount_b
        // (for a class project, this is sufficient)
        let lp_amount = amount_a_desired
            .checked_add(amount_b_desired)
            .ok_or(AmmError::MathOverflow)?;

        (amount_a_desired, amount_b_desired, lp_amount)
    } else {
        // Subsequent liquidity providers must add proportional to current reserves
        require!(reserve_a > 0 && reserve_b > 0, AmmError::MathOverflow);

        // Compute optimal B given A
        let amount_b_optimal = checked_div(checked_mul(amount_a_desired, reserve_b)?, reserve_a)?;
        let (amount_a, amount_b) = if amount_b_optimal <= amount_b_desired {
            (amount_a_desired, amount_b_optimal)
        } else {
            // Use B as the limiting factor
            let amount_a_optimal =
                checked_div(checked_mul(amount_b_desired, reserve_a)?, reserve_b)?;
            (amount_a_optimal, amount_b_desired)
        };

        // LP tokens to mint = min(
        //   amount_a * total_lp / reserve_a,
        //   amount_b * total_lp / reserve_b
        // )
        let lp_from_a = checked_div(checked_mul(amount_a, total_lp_supply)?, reserve_a)?;
        let lp_from_b = checked_div(checked_mul(amount_b, total_lp_supply)?, reserve_b)?;
        let lp_to_mint = lp_from_a.min(lp_from_b);

        (amount_a, amount_b, lp_to_mint)
    };

    // Transfer token A from user to vault
    let cpi_accounts_a = Transfer {
        from: user_token_a.to_account_info(),
        to: token_a_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx_a = CpiContext::new(token_program.to_account_info(), cpi_accounts_a);
    token::transfer(cpi_ctx_a, amount_a)?;

    // Transfer token B from user to vault
    let cpi_accounts_b = Transfer {
        from: user_token_b.to_account_info(),
        to: token_b_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx_b = CpiContext::new(token_program.to_account_info(), cpi_accounts_b);
    token::transfer(cpi_ctx_b, amount_b)?;

    // Mint LP tokens to user (signed by pool_authority PDA)
    let pool_key = pool.key();
    let seeds: &[&[u8]] = &[
        b"pool_authority",
        pool_key.as_ref(),
        &[pool.bump],
    ];
    let signer_seeds = &[seeds];

    let cpi_accounts_mint_lp = MintTo {
        mint: lp_mint.to_account_info(),
        to: user_lp_token.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info(),
    };
    let cpi_ctx_mint_lp =
        CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts_mint_lp, signer_seeds);
    token::mint_to(cpi_ctx_mint_lp, lp_to_mint)?;

    Ok(())
}