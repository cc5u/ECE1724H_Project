use std::rc::Rc;

use amm_dex::state::Pool;
use anchor_client::Program;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_lang::prelude::Pubkey;
use anchor_spl::associated_token::spl_associated_token_account::instruction::create_associated_token_account;
use anyhow::{Context, Result, anyhow};

use anchor_client::solana_sdk::signature::Signer;

use anchor_spl::token as spl_token;

type AnchorProgram = Program<Rc<Keypair>>;

pub fn load_pool(program: &AnchorProgram, pool: Pubkey) -> Result<Pool> {
    program
        .account(pool)
        .with_context(|| format!("Failed to fetch pool account {}", pool))
}

pub fn ensure_account_exists(
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

pub fn maybe_create_lp_ata(
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
