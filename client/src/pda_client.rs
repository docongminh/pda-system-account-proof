use std::ops::Deref;
use anyhow::Result;
use anchor_client::{ Client, anchor_lang::system_program };
use solana_sdk::{ signer::Signer, pubkey::Pubkey, sysvar };

use pda::accounts as pda_accounts;
use pda::instruction as pda_instruction;

const SEED: &[u8] = b"seed";

pub fn pda_program<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    user: Pubkey,
    program_id: Pubkey
) -> Result<()> {
    let program = client.program(program_id);
    let seed_signers = &[SEED, user.as_ref()];
    let (escrow_account, _) = Pubkey::find_program_address(seed_signers, &program_id);
    // Build and send a transaction.
    let signature = program
        .request()
        .accounts(pda_accounts::Initialize {
            escrow_account: escrow_account,
            owner: user,
            system_program: system_program::ID,
            rent: sysvar::ID,
        })
        .args(pda_instruction::Initialize{})
        .send()?;

    println!("init pda: {}", signature);

    Ok(())
}