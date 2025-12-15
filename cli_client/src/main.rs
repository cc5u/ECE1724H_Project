mod cli;
mod commands;

use crate::cli::*;
use crate::commands::*;
use clap::Parser;

use anchor_client::{Client, Cluster};

use anchor_client::solana_sdk::signature::read_keypair_file;
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
        }
        return Ok(());
    }

    // Interactive menu loop
    loop {
        println!("\nInteractive AMM DEX CLI — choose an option:");
        println!("1) InitPool");
        println!("2) AddLiquidity");
        println!("3) RemoveLiquidity");
        println!("4) Swap");
        println!("5) InspectPool");
        println!("6) ShowingDex");
        println!("q) Quit");

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
                let is_a_to_b = matches!(dir.to_lowercase().as_str(), "y" | "n" );
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
