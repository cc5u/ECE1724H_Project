use anchor_client::solana_sdk::signature::{Keypair};
use comfy_table::{presets::ASCII_FULL, ContentArrangement, Table};
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
       Some(token_a_balance / token_b_balance)
    } else {
        None
    };

    let mut table = Table::new();
    // Use full ASCII borders so each table is bounded top and bottom.
    table.load_preset(ASCII_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Field", "Value"]);
    table.add_row(vec!["Pool account".to_string(), pool_pubkey.to_string()]);
    table.add_row(vec!["Pool ID".to_string(), pool.pool_id.to_string()]);
    table.add_row(vec!["Token A mint".to_string(), pool.token_a_mint.to_string()]);
    table.add_row(vec![
        "Token A vault".to_string(),
        format!("{} (balance: {})", pool.token_a_vault, token_a_balance),
    ]);
    table.add_row(vec!["Token B mint".to_string(), pool.token_b_mint.to_string()]);
    table.add_row(vec![
        "Token B vault".to_string(),
        format!("{} (balance: {})", pool.token_b_vault, token_b_balance),
    ]);
    table.add_row(vec!["LP mint".to_string(), pool.lp_mint.to_string()]);
    table.add_row(vec!["LP supply".to_string(), lp_supply.to_string()]);
    table.add_row(vec!["Fee (bps)".to_string(), pool.fee_bps.to_string()]);
    table.add_row(vec![
        "Price A/B".to_string(),
        price.map(|p| format!("{:.6}", p))
            .unwrap_or_else(|| "N/A (zero balance)".to_string()),
    ]);

    println!("{table}\n");

    Ok(())
}