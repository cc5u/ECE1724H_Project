mod cli;
mod commands;

use crate::cli::*;
use crate::commands::*;
use clap::Parser;

use anchor_client::{Client, Cluster};

use anchor_client::solana_sdk::signature::read_keypair_file;
use std::rc::Rc;

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

    match cli.command {
        Commands::InitPool(args) => {
            cmd_init_pool(&program, &payer, args)?;
        }

        Commands::AddLiquidity(args) => {
            cmd_add_liquidity(&program, &payer, args)?;
        }

        Commands::RemoveLiquidity(args) => {
            cmd_rm_liquidity(&program, &payer, args)?;
        }

        Commands::Swap(args) => {
            cmd_swap(&program, &payer, args)?;
        }
    }
    Ok(())
}
