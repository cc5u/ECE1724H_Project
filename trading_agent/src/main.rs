use std::rc::Rc;

use amm_dex::state::Pool;
use anchor_client::{Client, Cluster, Program};
use anchor_client::solana_sdk::signature::{read_keypair_file, Keypair};
use anchor_lang::prelude::Pubkey;
use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = load_config(&cli.config)?;

    let cluster = match cfg.cluster.as_str() {
        "devnet" => Cluster::Devnet,
        "mainnet" => Cluster::Mainnet,
        "localnet" | _ => Cluster::Localnet,
    };

    let keypair_path = shellexpand::tilde(&cfg.keypair).to_string();
    let payer = read_keypair_file(&keypair_path).expect("Failed to read keypair file");
    let payer = Rc::new(payer);

    let client = Client::new_with_options(cluster, payer.clone(), Default::default());
    let program = client.program(amm_dex::id())?;

    let pools: Vec<(Pubkey, Pool)> = program.accounts::<Pool>(vec![]).await?;
    println!("Found {} pool(s) for program {}", pools.len(), program.id());
    let mut snapshots = Vec::with_capacity(pools.len());

    for (address, pool) in pools {
        match fetch_pool_snapshot(&program, address, pool).await {
            Ok(snapshot) => snapshots.push(snapshot),
            Err(err) => eprintln!("Failed to fetch pool {address}: {err:?}"),
        }
    }

    for (idx, snapshot) in snapshots.iter().enumerate() {
        let pool = &snapshot.pool;
        println!("{}: {} | pool_id: {} | token_a_mint: {} | token_b_mint: {} | lp_mint: {} | fee_bps: {} | reserve_a: {} | reserve_b: {}",
            idx + 1,
            snapshot.address,
            pool.pool_id,
            pool.token_a_mint,
            pool.token_b_mint,
            pool.lp_mint,
            pool.fee_bps,
            snapshot.reserve_a,
            snapshot.reserve_b,
        );
    }
    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, default_value = "Agent.toml")]
    config: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
struct Config {
    cluster: String,
    keypair: String,

    pools: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cluster: "localnet".to_string(),
            keypair: "~/.config/solana/id.json".to_string(),

            pools: Vec::new(),
        }
    }
}

fn load_config(path: &str) -> Result<Config> {
    let path = shellexpand::tilde(path).to_string();
    let raw = std::fs::read_to_string(&path).with_context(|| format!("read config {}", path))?;
    toml::from_str(&raw).with_context(|| format!("parse toml {}", path))
}

#[derive(Clone)]
struct PoolSnapshot{
    address: Pubkey,
    pool: amm_dex::state::Pool,
    reserve_a: u64,
    reserve_b: u64,
}

async fn fetch_pool_snapshot(
    program: &Program<Rc<Keypair>>,
    address: Pubkey,
    pool_state: Pool,
) -> Result<PoolSnapshot> {
    let rpc = program.rpc();
    // Using "amount" instead of "ui_amount" since pool reserves need exact precision for AMM calculations.
    //
    // - amount: The raw balance as a string in the smallest unit (lamports for SOL, or token's
    //   base unit). Parsed as u64. This is precise for on-chain operations.
    // - ui_amount: A human-readable decimal string adjusted by the token's decimals.
    //   For example, if a token has 6 decimals and the amount is 1,000,000, ui_amount would be "1.0".
    //   This is useful for display but loses precision and requires re-conversion for contract calls.
    let reserve_a = rpc
        .get_token_account_balance(&pool_state.token_a_vault)
        .await?
        .amount
        .parse::<u64>()
        .context("parse reserve_a")?;
    let reserve_b = rpc
        .get_token_account_balance(&pool_state.token_b_vault)
        .await?
        .amount
        .parse::<u64>()
        .context("parse reserve_b")?;

    Ok(PoolSnapshot {
        address,
        pool: pool_state,
        reserve_a,
        reserve_b,
    })
}
