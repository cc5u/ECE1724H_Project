mod cli;
mod commands;

use crate::cli::*;
use crate::commands::*;
use anchor_client::solana_sdk::signer::Signer;
use clap::Parser;
use anchor_client::{Client, Cluster};

use anchor_client::solana_sdk::signature::read_keypair_file;
use colored::Colorize;
use std::rc::Rc;
use std::io::{stdin, stdout, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = stdout().flush();
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() -> anyhow::Result<()> {
    let cli = cli::CliArgs::parse();

    let cluster = match cli.cluster.as_str() {
        "devnet" => Cluster::Devnet,
        "mainnet" => Cluster::Mainnet,
        "localnet" | _ => Cluster::Localnet,
    };

    let keypair_path = shellexpand::tilde(&cli.keypair).to_string();
    let payer = read_keypair_file(&keypair_path).expect("Failed to read keypair file");
    let payer = Rc::new(payer);

    let client = Client::new_with_options(cluster, payer.clone(), Default::default());
    let program = client.program(amm_dex::id())?;

    if let Some(command) = cli.command {
        match command {
            Commands::InitPool(args) => cmd_init_pool(&program, &payer, args)?,
            Commands::AddLiquidity(args) => cmd_add_liquidity(&program, &payer, args)?,
            Commands::RemoveLiquidity(args) => cmd_rm_liquidity(&program, &payer, args)?,
            Commands::Swap(args) => cmd_swap(&program, &payer, args)?,
            Commands::InspectPool(args) => cmd_inspect_pool(&program, args)?,
            Commands::ShowingDex(args) => cmd_showing_dex(&program, args)?,
            Commands::Wallet(_) => cmd_wallet(&program)?,
        }
        return Ok(());
    }

    // Interactive menu loop
    loop {

        let balance_lamports = match program.rpc().get_balance(&payer.pubkey()) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to fetch balance: {}", e);
                0
            }
        };
        let balance_sol = (balance_lamports as f64) / 1_000_000_000.0;

        // Build menu lines
        let wallet_line = format!("Wallet: {}", keypair_path);
        let balance_line = format!("Balance: {:.6} SOL", balance_sol);
        let menu_lines = [
            "1) InitPool".normal(),
            "2) AddLiquidity".normal(),
            "3) RemoveLiquidity".normal(),
            "4) Swap".normal(),
            "5) InspectPool".normal(),
            "6) ShowingDex".normal(),
            "7) Wallet (SOL + tokens)".normal(),
            "q) Quit".red(),
            " ".normal(),
        ];

        // Determine box width from the longest line
        let title = " Interactive AMM DEX CLI ".green();
        let mut width = title.len() + 40;
        for l in menu_lines.iter() {
            width = width.max(l.len());
        }

        let border = "=".repeat(width);
        let pad = width.saturating_sub(title.len());
        let left = pad / 2;
        let right = pad.saturating_sub(left);
        let title_line = format!("{}{}{}", "-".repeat(left), title, "-".repeat(right));

        println!("\n{}", border);
        println!("{}", title_line);
        println!("{}", border);
        println!("{}", wallet_line);
        println!("{}", balance_line);
        println!("\n- choose an option:");
        for l in menu_lines.iter() {
            println!("{}", l);
        }

        let choice = read_input("> ");
        match choice.as_str() {
            "1" => {
                let token_a = read_input("token_a_mint: ");
                let token_b = read_input("token_b_mint: ");
                let fee = read_input("fee_bps (number, default 30): ");
                let fee_val = fee.parse::<u16>().unwrap_or(30);
                let args = InitPoolArgs {
                    token_a_mint: token_a,
                    token_b_mint: token_b,
                    fee_bps: fee_val,
                };
                if let Err(e) = cmd_init_pool(&program, &payer, args) {
                    eprintln!("InitPool error: {}", e);
                }
            }

            "2" => {
                let pool = read_input("pool: ");
                let amount_a = read_input("amount_a (u64): ");
                let amount_b = read_input("amount_b (u64): ");
                let a = match amount_a.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => { eprintln!("Invalid amount_a"); continue; }
                };
                let b = match amount_b.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => { eprintln!("Invalid amount_b"); continue; }
                };
                let args = AddLiquidityArgs { pool, amount_a: a, amount_b: b };
                if let Err(e) = cmd_add_liquidity(&program, &payer, args) {
                    eprintln!("AddLiquidity error: {}", e);
                }
            }

            "3" => {
                let pool = read_input("pool: ");
                let lp = read_input("lp_amount (u64): ");
                let lp_amount = match lp.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => { eprintln!("Invalid lp_amount"); continue; }
                };
                let args = RemoveLiquidityArgs { pool, lp_amount };
                if let Err(e) = cmd_rm_liquidity(&program, &payer, args) {
                    eprintln!("RemoveLiquidity error: {}", e);
                }
            }

            "4" => {
                let pool = read_input("pool: ");
                let amount_in = read_input("amount_in (u64): ");
                let minimum_out = read_input("minimum_out (u64): ");
                let dir = read_input("direction (A->B type 'y' else 'n'): ");
                let a_in = match amount_in.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => { eprintln!("Invalid amount_in"); continue; }
                };
                let min_out = match minimum_out.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => { eprintln!("Invalid minimum_out"); continue; }
                };
                let is_a_to_b = matches!(dir.trim().to_lowercase().as_str(), "y");
                let args = SwapArgs { pool, amount_in: a_in, minimum_out: min_out, is_a_to_b };
                if let Err(e) = cmd_swap(&program, &payer, args) {
                    eprintln!("Swap error: {}", e);
                }
            }

            "5" => {
                let pool = read_input("pool: ");
                let args = InspectPoolArgs { pool };
                if let Err(e) = cmd_inspect_pool(&program, args) {
                    eprintln!("InspectPool error: {}", e);
                }
            }

            "6" => {
                let args = ShowingDexArgs {};
                if let Err(e) = cmd_showing_dex(&program, args) {
                    eprintln!("ShowingDex error: {}", e);
                }
            }

            "7" => {
                if let Err(e) = cmd_wallet(&program) {
                    eprintln!("Wallet error: {}", e);
                }
            }

            "q" | "Q" => {
                println!("Quitting.");
                break;
            }

            other => {
                println!("Unrecognized choice: {}", other);
            }
        }
    }

    Ok(())
}
