use anchor_client::solana_sdk::signature::{Keypair, Signer};
use std::rc::Rc;

use anchor_lang::prelude::Pubkey;

use amm_dex::accounts as amm_accounts;
use amm_dex::instruction as amm_ix;
use anchor_client::Program;
use anchor_spl::token as spl_token;
use solana_system_interface::program;

use crate::cli::*;

pub fn cmd_init_pool(
    program: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    args: InitPoolArgs,
) -> anyhow::Result<()> {
    // parse pubkeys
    let token_a_mint = args.token_a_mint.parse::<Pubkey>()?;
    let token_b_mint = args.token_b_mint.parse::<Pubkey>()?;
    let fee_bps = args.fee_bps;

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
            system_program: Pubkey::new_from_array(program::ID.to_bytes()),
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
