use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dex-cli", about = "Rust CLI wallet for AMM DEX")]
pub struct CliArgs{
    // Solana cluster: localnet, devnet, mainnet
    #[arg(long, default_value = "localnet")]
    pub cluster: String,

    // Path to keypair file
    #[arg(long, default_value = "~/.config/solana/id.json")]
    pub keypair: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // Initialize a new AMM pool
    InitPool {
        #[arg(long)]
        token_a_mint: String,
        #[arg(long)]
        token_b_mint: String,
        #[arg(long, default_value_t = 30)]
        fee_bps: u16,
    }
}