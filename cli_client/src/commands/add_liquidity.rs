use std::rc::Rc;

use crate::cli::AddLiquidityArgs;

use amm_dex::{accounts as amm_accounts, instruction as amm_ix, state::Pool};
use anchor_client::Program;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signature::Signer;
use anchor_lang::prelude::Pubkey;
use anchor_spl::associated_token::{
    get_associated_token_address,
    spl_associated_token_account::instruction::create_associated_token_account,
};
use anchor_spl::token as spl_token;
use anyhow::{Context, Result, anyhow};

type AnchorProgram = Program<Rc<Keypair>>;

pub fn cmd_add_liquidity(
    program: &Program<Rc<Keypair>>,
    payer: &Rc<Keypair>,
    args: AddLiquidityArgs,
) -> Result<()> {
    let pool_pubkey = args.pool.parse::<Pubkey>()?;
    let pool = load_pool(program, pool_pubkey)?;

    let (pool_authority, _bump) =
        Pubkey::find_program_address(&[b"pool_authority", pool_pubkey.as_ref()], &program.id());

    let user = payer.pubkey();
    let user_token_a = get_associated_token_address(&user, &pool.token_a_mint);
    let user_token_b = get_associated_token_address(&user, &pool.token_b_mint);
    let user_lp_token = get_associated_token_address(&user, &pool.lp_mint);

    // Fail early with clear instructions if required token accounts are missing.
    ensure_account_exists(
        program,
        &user_token_a,
        "User token A ATA",
        &pool.token_a_mint,
    )?;
    ensure_account_exists(
        program,
        &user_token_b,
        "User token B ATA",
        &pool.token_b_mint,
    )?;
    let lp_ata_ix = maybe_create_lp_ata(program, payer, &user, &user_lp_token, &pool.lp_mint)?;
    let created_lp_ata = lp_ata_ix.is_some();

    let mut request = program.request();
    if let Some(ix) = lp_ata_ix {
        request = request.instruction(ix);
    }

    let sig = request
        .accounts(amm_accounts::AddLiquidity {
            pool: pool_pubkey,
            pool_authority,
            token_a_vault: pool.token_a_vault,
            token_b_vault: pool.token_b_vault,
            user_token_a,
            user_token_b,
            lp_mint: pool.lp_mint,
            user_lp_token,
            user,
            token_program: spl_token::ID,
        })
        .args(amm_ix::AddLiquidity {
            amount_a_desired: args.amount_a,
            amount_b_desired: args.amount_b,
        })
        .send()?;

    println!("Added liquidity to pool {} tx: {}", pool_pubkey, sig);
    if created_lp_ata {
        println!("Created user LP ATA: {}", user_lp_token);
    }
    println!("User token A ATA: {}", user_token_a);
    println!("User token B ATA: {}", user_token_b);
    println!("User LP ATA: {}", user_lp_token);
    Ok(())
}

fn load_pool(program: &AnchorProgram, pool: Pubkey) -> Result<Pool> {
    program
        .account(pool)
        .with_context(|| format!("Failed to fetch pool account {}", pool))
}

fn ensure_account_exists(
    program: &AnchorProgram,
    address: &Pubkey,
    label: &str,
    mint: &Pubkey,
) -> Result<()> {
    match program.rpc().get_account(address) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!(
            "{} ({}) is missing. Create it first (e.g. `spl-token create-account {}`) and retry.",
            label,
            address,
            mint
        )),
    }
}

fn maybe_create_lp_ata(
    program: &AnchorProgram,
    payer: &Rc<Keypair>,
    user: &Pubkey,
    user_lp_token: &Pubkey,
    lp_mint: &Pubkey,
) -> Result<Option<Instruction>> {
    match program.rpc().get_account(user_lp_token) {
        Ok(_) => Ok(None),
        Err(_) => {
            let ix =
                create_associated_token_account(&payer.pubkey(), user, lp_mint, &spl_token::ID);
            Ok(Some(ix))
        }
    }
}
