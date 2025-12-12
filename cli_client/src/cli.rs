use clap::{Args, Parser, Subcommand};

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

#[derive(Args, Debug)]
pub struct InitPoolArgs {
    #[arg(long)]
    pub token_a_mint: String,
    #[arg(long)]
    pub token_b_mint: String,
    #[arg(long, default_value_t = 30)]
    pub fee_bps: u16,
}

#[derive(Args, Debug)]
pub struct AddLiquidityArgs {
    #[arg(long)]
    pub pool: String,
    #[arg(long)]
    pub amount_a: u64,
    #[arg(long)]
    pub amount_b: u64,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // Initialize a new AMM pool
    InitPool(InitPoolArgs),

    // Add liquidity to an existing pool
    AddLiquidity(AddLiquidityArgs),
}