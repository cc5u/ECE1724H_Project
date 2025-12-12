use anchor_client::{
    Client,
    Cluster,
};

use anchor_client::solana_sdk::signature::{Keypair, read_keypair_file, Signer};
use std::rc::Rc;

use clap::{Parser, Subcommand};

use anchor_lang::prelude::Pubkey;
use shellexpand;

use anchor_client::Program;
use anchor_client::solana_sdk::system_program;
use anchor_spl::token as spl_token;
use amm_dex::accounts as amm_accounts;
use amm_dex::instruction as amm_ix;
fn main() -> anyhow::Result<()>{

    let args = CliArgs::parse();

    let cluster = match args.cluster.as_str() {
        "devnet" => Cluster::Devnet,
        "mainnet" => Cluster::Mainnet,
        "localnet" | _ => Cluster::Localnet,
    };

    let keypair_path = shellexpand::tilde(&args.keypair).to_string();
    let payer = read_keypair_file(&keypair_path)
        .expect("Failed to read keypair file");
    let payer = Rc::new(payer);

    let client = Client::new_with_options(cluster, payer.clone(), Default::default());
    let program = client.program(amm_dex::id())?;

    match args.command {
        Commands::InitPool { token_a_mint, token_b_mint, fee_bps } => {
            cmd_init_pool(&program, &payer, &token_a_mint, &token_b_mint, fee_bps)?;
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "dex-cli", about = "Rust CLI wallet for AMM DEX")]
struct CliArgs{
    // Solana cluster: localnet, devnet, mainnet
    #[arg(long, default_value = "localnet")]
    cluster: String,

    // Path to keypair file
    #[arg(long, default_value = "~/.config/solana/id.json")]
    keypair: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

fn cmd_init_pool(
    program: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    token_a_mint_str: &str,
    token_b_mint_str: &str,
    fee_bps: u16,
) -> anyhow::Result<()> {
    // parse pubkeys
    let token_a_mint = token_a_mint_str.parse::<Pubkey>()?;
    let token_b_mint = token_b_mint_str.parse::<Pubkey>()?;

    // derive PDAs
    let (pool_pda, _bump_pool) = Pubkey::find_program_address(
        &[b"pool", token_a_mint.as_ref(), token_b_mint.as_ref()],
        &program.id(),
    );
    let (pool_authority, _bump_auth) =
        Pubkey::find_program_address(&[b"pool_authority", pool_pda.as_ref()], &program.id());

    // new keypairs for token vaults and LP mint (they will be created by the program)
    let token_a_vault = Keypair::new();
    let token_b_vault = Keypair::new();
    let lp_mint = Keypair::new();

    // build and send the transaction
    let tx = program
        .request()
        .accounts(amm_accounts::InitializePool {
            pool: pool_pda,
            pool_authority,
            token_a_mint,
            token_b_mint,
            token_a_vault: token_a_vault.pubkey(),
            token_b_vault: token_b_vault.pubkey(),
            lp_mint: lp_mint.pubkey(),
            payer: payer.pubkey(),
            system_program: system_program::id(),
            token_program: spl_token::ID,
            rent: anchor_client::solana_sdk::sysvar::rent::id(),
        })
        .args(amm_ix::InitializePool { fee_bps })
        .signer(&token_a_vault)
        .signer(&token_b_vault)
        .signer(&lp_mint)
        .send()?;

    println!("Initialized pool {} tx: {}", pool_pda, tx);
    println!("Pool PDA          : {pool_pda}");
    println!("Pool authority PDA: {pool_authority}");
    println!("Token A mint      : {token_a_mint}");
    println!("Token B mint      : {token_b_mint}");
    println!("Token A vault     : {}", token_a_vault.pubkey());
    println!("Token B vault     : {}", token_b_vault.pubkey());
    println!("LP mint           : {}", lp_mint.pubkey());


    Ok(())
}






