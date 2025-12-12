use crate::error::AmmError;
use crate::state::*;
use crate::utils::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// CHECK
    #[account(
        seeds = [b"pool_authority", pool.key().as_ref()],
        bump = pool.authority_bump,
    )]
    pub pool_authority: UncheckedAccount<'info>,

    // Vaults
    #[account(mut)]
    pub token_a_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b_vault: Account<'info, TokenAccount>,

    // User accounts
    #[account(mut)]
    pub user_source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_destination: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn swap(
    ctx: Context<Swap>,
    amount_in: u64,
    minimum_out: u64,
    is_a_to_b: bool,
) -> Result<()> {
    // 1. Determine input/output vaults based on direction.
    // 2. Compute output_amount using x*y=k with fee:
    //      amount_in_after_fee = amount_in * (1_0000 - fee_bps) / 1_0000;
    //      new_reserve_in = reserve_in + amount_in_after_fee;
    //      new_reserve_out = k / new_reserve_in;
    //      amount_out = old_reserve_out - new_reserve_out;
    // 3. Check amount_out >= minimum_out for slippage protection.
    // 4. Transfer amount_in from user to input vault.
    // 5. Transfer amount_out from output vault to user.

    msg!("Starting swap, amount_in = {}", amount_in);

    let pool = &ctx.accounts.pool;
    let token_program = &ctx.accounts.token_program;

    let (input_vault, output_vault) = if is_a_to_b {
        (&ctx.accounts.token_a_vault, &ctx.accounts.token_b_vault)
    } else {
        (&ctx.accounts.token_b_vault, &ctx.accounts.token_a_vault)
    };

    let user_source = &ctx.accounts.user_source;
    let user_destination = &ctx.accounts.user_destination;

    let reserve_in = input_vault.amount;
    let reserve_out = output_vault.amount;

    // Compute amount_out via constant-product formula with fee
    let amount_out = get_amount_out(amount_in, reserve_in, reserve_out, pool.fee_bps)?;

    // Slippage protection
    require!(
        amount_out >= minimum_out,
        AmmError::InsufficientOutputAmount
    );

    // 1. Transfer amount_in from user -> input_vault
    let cpi_accounts_in = Transfer {
        from: user_source.to_account_info(),
        to: input_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx_in = CpiContext::new(token_program.to_account_info(), cpi_accounts_in);
    token::transfer(cpi_ctx_in, amount_in)?;

    // 2. Transfer amount_out from output_vault -> user_destination (signed by pool_authority)
    let pool_key = pool.key();
    let seeds: &[&[u8]] = &[b"pool_authority", pool_key.as_ref(), &[pool.authority_bump]];
    let signer_seeds = &[seeds];

    let cpi_accounts_out = Transfer {
        from: output_vault.to_account_info(),
        to: user_destination.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info(),
    };
    let cpi_ctx_out = CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts_out,
        signer_seeds,
    );
    token::transfer(cpi_ctx_out, amount_out)?;

    msg!("Swap finished, amount_out = {}", amount_out);

    Ok(())
}
