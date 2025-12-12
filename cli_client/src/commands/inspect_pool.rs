use anchor_client::solana_sdk::signature::{Keypair};
use std::rc::Rc;

use anchor_lang::prelude::Pubkey;
use anchor_client::Program;

use crate::cli::*;
use crate::commands::utils::load_pool;

pub fn cmd_inspect_pool(
    program: &Program<Rc<Keypair>>,
    args: InspectPoolArgs,
) -> anyhow::Result<()>{
    let pool_pubkey = args.pool.parse::<Pubkey>()?;
    let pool = load_pool(program, pool_pubkey)?;

    
    let token_a_balance = program.rpc().get_token_account_balance(&pool.token_a_vault)?.ui_amount.ok_or(anyhow::anyhow!("No UI amount for token A vault"))?;
    let token_b_balance = program.rpc().get_token_account_balance(&pool.token_b_vault)?.ui_amount.ok_or(anyhow::anyhow!("No UI amount for token B vault"))?;
    let lp_supply = program.rpc().get_token_supply(&pool.lp_mint)?.ui_amount.ok_or(anyhow::anyhow!("No UI amount for LP mint"))?;

    let price = if token_b_balance > 0.0 {
        token_a_balance / token_b_balance
    } else {
        0.0
    };

    println!("Pool: {}", pool_pubkey);
    println!("========================================");
    println!("Token A mint  : {}", pool.token_a_mint);
    println!(
        "Token A vault : {} (balance: {})",
        pool.token_a_vault,
        &token_a_balance
    );
    println!("========================================");
    println!("Token B mint  : {}", pool.token_b_mint);
    
    println!(
        "Token B vault : {} (balance: {})",
        pool.token_b_vault,
        &token_b_balance
    );
    println!("========================================");
    println!("LP mint       : {}", pool.lp_mint);
    println!(
        "LP supply     : {}",
        &lp_supply,
    );
    println!("Fee (bps)     : {}", pool.fee_bps);
    if price > 0.0 {
        println!("Price A/B     : {:.6}", price);
    } else {
        println!("Price A/B     : N/A (zero balance)");
    }
    Ok(())
}