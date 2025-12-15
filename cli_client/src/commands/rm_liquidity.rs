use std::rc::Rc;

use crate::cli::RemoveLiquidityArgs;
use crate::commands::utils::*;

use amm_dex::{accounts as amm_accounts, instruction as amm_ix};
use anchor_client::Program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signature::Signer;
use anchor_lang::prelude::Pubkey;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token as spl_token;
use anyhow::Result;

pub fn cmd_rm_liquidity(
    program: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    args: RemoveLiquidityArgs,
) -> Result<()> {
    let program = program;
    let pool_pubkey = args.pool.parse::<Pubkey>()?;
    let pool = load_pool(program, pool_pubkey)?;

    let (pool_authority, _bump) =
        Pubkey::find_program_address(&[b"pool_authority", pool_pubkey.as_ref()], &program.id());

    let user = payer.pubkey();
    let user_token_a = get_associated_token_address(&user, &pool.token_a_mint);
    let user_token_b = get_associated_token_address(&user, &pool.token_b_mint);
    let user_lp_token = get_associated_token_address(&user, &pool.lp_mint);

    ensure_account_exists(program, &user_lp_token, "User LP token ATA", &pool.lp_mint)?;
    ensure_account_exists(
        program,
        &user_token_a,
        "User token A ATA",
        &pool.token_a_mint,
    )?;
    ensure_account_exists(
        program,
        &user_token_b,
        "User token B ATA",
        &pool.token_b_mint,
    )?;

    let sig = program
        .request()
        .accounts(amm_accounts::RemoveLiquidity {
            pool: pool_pubkey,
            pool_authority,
            token_a_vault: pool.token_a_vault,
            token_b_vault: pool.token_b_vault,
            lp_mint: pool.lp_mint,
            user_lp_token,
            user_token_a,
            user_token_b,
            user,
            token_program: spl_token::ID,
        })
        .args(amm_ix::RemoveLiquidity {
            lp_amount: args.lp_amount,
        })
        .send()?;

    println!(
        "Removed {} LP from pool\n{} tx: {}",
        args.lp_amount, pool_pubkey, sig
    );
    println!("User token A ATA: {}", user_token_a);
    println!("User token B ATA: {}", user_token_b);
    println!("User LP ATA: {}", user_lp_token);
    Ok(())
}
