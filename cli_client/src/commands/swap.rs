use std::rc::Rc;

use crate::cli::SwapArgs;

use amm_dex::{accounts as amm_accounts, instruction as amm_ix, state::Pool};
use anchor_client::Program;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_lang::prelude::Pubkey;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token as spl_token;
use anyhow::{Context, Result, anyhow};

type AnchorProgram = Program<Rc<Keypair>>;

pub fn cmd_swap(program: &Program<Rc<Keypair>>, payer: &Rc<Keypair>, args: SwapArgs) -> Result<()> {
    let program = program;
    let pool_pubkey = args.pool.parse::<Pubkey>()?;
    let pool = load_pool(program, pool_pubkey)?;

    let (pool_authority, _bump) =
        Pubkey::find_program_address(&[b"pool_authority", pool_pubkey.as_ref()], &program.id());

    let user = payer.pubkey();

    // Decide direction and derive ATAs
    let (user_source, user_destination, direction_label) = if args.is_a_to_b {
        let src = get_associated_token_address(&user, &pool.token_a_mint);
        let dst = get_associated_token_address(&user, &pool.token_b_mint);
        (src, dst, "A -> B")
    } else {
        let src = get_associated_token_address(&user, &pool.token_b_mint);
        let dst = get_associated_token_address(&user, &pool.token_a_mint);
        (src, dst, "B -> A")
    };

    ensure_account_exists(program, &user_source, "Source ATA")?;
    ensure_account_exists(program, &user_destination, "Destination ATA")?;

    let sig = program
        .request()
        .accounts(amm_accounts::Swap {
            pool: pool_pubkey,
            pool_authority,
            token_a_vault: pool.token_a_vault,
            token_b_vault: pool.token_b_vault,
            user_source,
            user_destination,
            user,
            token_program: spl_token::ID,
        })
        .args(amm_ix::Swap {
            amount_in: args.amount_in,
            minimum_out: args.minimum_out,
            is_a_to_b: args.is_a_to_b,
        })
        .send()?;

    println!(
        "Swap {} complete.\nPool: {}\ntx: {}",
        direction_label, pool_pubkey, sig
    );
    println!("Source ATA: {}", user_source);
    println!("Destination ATA: {}", user_destination);

    Ok(())
}

fn load_pool(program: &AnchorProgram, pool: Pubkey) -> Result<Pool> {
    program
        .account(pool)
        .with_context(|| format!("Failed to fetch pool account {}", pool))
}

fn ensure_account_exists(program: &AnchorProgram, address: &Pubkey, label: &str) -> Result<()> {
    match program.rpc().get_account(address) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!(
            "{} ({}) is missing. Create it first and retry.",
            label,
            address
        )),
    }
}
