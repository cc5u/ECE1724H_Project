use std::rc::Rc;

use anchor_client::solana_account_decoder::UiAccountData;
use anchor_client::Program;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_lang::prelude::Pubkey;
use anchor_spl::token as spl_token;
use anyhow::Result;
use serde_json::Value;
use solana_client::rpc_request::TokenAccountsFilter;

/// Show SOL balance and all SPL token balances (ATAs) for the payer.
pub fn cmd_wallet(program: &Program<Rc<Keypair>>) -> Result<()> {
    let rpc = program.rpc();
    let owner: &Pubkey = &program.payer();

    // SOL balance
    let lamports = rpc.get_balance(owner)?;
    let sol = lamports as f64 / 1_000_000_000.0;
    println!("Wallet: {} | SOL: {:.6}", owner, sol);

    // Token accounts (ATAs)
    let resp = rpc.get_token_accounts_by_owner(owner, TokenAccountsFilter::ProgramId(spl_token::ID))?;
    if resp.is_empty() {
        println!("No token accounts found.");
        return Ok(());
    }

    println!("Token accounts (ATAs):");
    let mut table = comfy_table::Table::new();
    table
        .set_header(vec!["Token", "Mint", "Amount"])
        .load_preset(comfy_table::presets::ASCII_BORDERS_ONLY);

    for keyed in resp {
        if let UiAccountData::Json(parsed) = keyed.account.data {
            if parsed.program != "spl-token" {
                continue;
            }
            if let Some(info) = parsed.parsed.get("info") {
                let mint = info.get("mint").and_then(Value::as_str).unwrap_or("?");
                let amount_ui = info
                    .get("tokenAmount")
                    .and_then(|t| t.get("uiAmountString"))
                    .and_then(Value::as_str)
                    .unwrap_or("0");
                let raw_amount = info
                    .get("tokenAmount")
                    .and_then(|t| t.get("amount"))
                    .and_then(Value::as_str)
                    .unwrap_or("0");
                let decimals = info
                    .get("tokenAmount")
                    .and_then(|t| t.get("decimals"))
                    .and_then(Value::as_u64)
                    .unwrap_or(0);

                let amount_display = format!("{} (raw: {}, decimals: {})", amount_ui, raw_amount, decimals);
                table.add_row(vec![keyed.pubkey.to_string(), mint.to_string(), amount_display]);
            }
        }
    }

    println!("{table}");

    Ok(())
}
