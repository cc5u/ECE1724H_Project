use std::rc::Rc;

use amm_dex::state::Pool;
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_lang::prelude::Pubkey;

fn main() -> anyhow::Result<()> {
    let cluster = Cluster::Localnet;
    let keypair_path = shellexpand::tilde("~/.config/solana/id.json").to_string();
    let payer = read_keypair_file(&keypair_path).expect("Failed to read keypair file");
    let payer = Rc::new(payer);

    let client = Client::new_with_options(cluster, payer.clone(), Default::default());
    let program = client.program(amm_dex::id())?;

    let pools: Vec<(Pubkey, Pool)> = program.accounts::<Pool>(vec![])?;
    println!("Found {} pool(s) for program {}", pools.len(), program.id());
    for (idx, (address, pool)) in pools.iter().enumerate() {
        println!("{}: {} | pool_id: {} | token_a_mint: {} | token_b_mint: {} | lp_mint: {} | fee_bps: {}",
            idx + 1,
            address,
            pool.pool_id,
            pool.token_a_mint,
            pool.token_b_mint,
            pool.lp_mint,
            pool.fee_bps,
        );
    }

    Ok(())
}
