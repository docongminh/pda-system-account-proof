use std::{ rc::Rc };
use anyhow::Result;
use anchor_client::{ Client, Cluster };
use clap::Parser;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::read_keypair_file,
    pubkey::Pubkey,
    signer::Signer,
};

mod pda_client;
mod hacker_client;
use crate::pda_client::pda_program;
use crate::hacker_client::hacker_program;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    pda_program_id: Pubkey,
    #[clap(long)]
    hacker_program_id: Pubkey,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // Wallet and cluster params.
    let payer = read_keypair_file(
        &*shellexpand::tilde("/Users/minhdo/.config/solana/id.json")
    ).expect("Example requires a keypair file");

    let hacker = read_keypair_file(
        &*shellexpand::tilde("/Users/minhdo/.config/solana/id.json")
    ).expect("Example requires a keypair file");
    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string()
    );
    // Client.
    let payer = Rc::new(payer);
    let client = Client::new_with_options(
        url.clone(),
        payer.clone(),
        CommitmentConfig::processed()
    );

    // pda program: init a pda account and transfer 1 SOL to this pda account
    pda_program(&client, payer.pubkey(), opts.pda_program_id)?;

    // hacker program: know seeds, program id, and re-create pda
    // re-create new program and use this pda to hack token inside account
    hacker_program(&client, hacker.pubkey(), payer.pubkey(), opts.hacker_program_id, opts.pda_program_id)?;

    Ok(())
}