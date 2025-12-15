use crate::error::AmmError;
use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// CHECK
    #[account(
        seeds = [b"pool_authority", pool.key().as_ref()],
        bump = pool.authority_bump,
    )]
    pub pool_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_a_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_lp_token.mint == lp_mint.key(),
        constraint = user_lp_token.owner == user.key(),
    )]
    pub user_lp_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
    // 1. Calculate share of reserves (token A + token B) based on lp_amount / total_lp_supply.
    // 2. Burn LP tokens from user (token::burn).
    // 3. Transfer corresponding amounts from vaults back to user (token::transfer).

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

    require!(total_lp_supply > 0, AmmError::InvalidLpSupply);

    let lp_u128 = lp_amount as u128;
    let reserve_a_u128 = reserve_a as u128;
    let reserve_b_u128 = reserve_b as u128;
    let total_lp_u128 = total_lp_supply as u128;

    // User's share: amount = lp_amount / total_lp * reserve
    let amount_a_u128 = lp_u128
        .checked_mul(reserve_a_u128)
        .and_then(|n| n.checked_div(total_lp_u128))
        .ok_or(AmmError::MathOverflow)?;
    let amount_b_u128 = lp_u128
        .checked_mul(reserve_b_u128)
        .and_then(|n| n.checked_div(total_lp_u128))
        .ok_or(AmmError::MathOverflow)?;

    let amount_a = u64::try_from(amount_a_u128).map_err(|_| AmmError::MathOverflow)?;
    let amount_b = u64::try_from(amount_b_u128).map_err(|_| AmmError::MathOverflow)?;

    // Burn LP from user
    let cpi_accounts_burn = Burn {
        mint: lp_mint.to_account_info(),
        from: user_lp_token.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx_burn = CpiContext::new(token_program.to_account_info(), cpi_accounts_burn);
    token::burn(cpi_ctx_burn, lp_amount)?;

    // Transfer A/B from vaults to user (signed by pool_authority)
    let pool_key = pool.key();
    let seeds: &[&[u8]] = &[b"pool_authority", pool_key.as_ref(), &[pool.authority_bump]];
    let signer_seeds = &[seeds];

    let cpi_accounts_a = Transfer {
        from: token_a_vault.to_account_info(),
        to: user_token_a.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info(),
    };
    let cpi_ctx_a = CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts_a,
        signer_seeds,
    );
    token::transfer(cpi_ctx_a, amount_a)?;

    let cpi_accounts_b = Transfer {
        from: token_b_vault.to_account_info(),
        to: user_token_b.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info(),
    };
    let cpi_ctx_b = CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts_b,
        signer_seeds,
    );
    token::transfer(cpi_ctx_b, amount_b)?;
    Ok(())
}
