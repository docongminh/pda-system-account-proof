use anchor_lang::prelude::*;

declare_id!("DzSN8ZCEURUe3nXtUi23L4N6jCQT16B8GjfZQ1CkoQEh");
const LAMPORT_PER_SOL: u64 = 1_000_000_000;
const SEED: &[u8] = b"seed";
#[program]
pub mod pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("escrow_account").unwrap();
        ctx.accounts.create_native_account_vault(bump)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    #[account(mut,
        seeds=[SEED, owner.key().as_ref()],
        bump
    )]
    pub escrow_account: AccountInfo<'info>,
    #[account(mut, constraint = owner.lamports() > 0 && owner.data_is_empty())]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize <'info> {
    fn create_native_account_vault(&self, bump: u8) -> Result<()> {
        let authority_key = self.owner.key();
        let seed_signers = &[&[SEED, authority_key.as_ref(), bytemuck::bytes_of(&bump)][..]];

        let lamports = self.rent.minimum_balance(0);
        anchor_lang::solana_program::program::invoke_signed(
            &anchor_lang::solana_program::system_instruction::create_account(
                self.owner.key,
                self.escrow_account.key,
                lamports,
                0,
                &self.system_program.key() // assign vault account for system program rather than itself program id
            ),
            &[
                self.owner.to_account_info(),
                self.escrow_account.to_account_info(),
                self.system_program.to_account_info(),
            ],
            seed_signers
        )?;

        let transfer_sol_instruction = anchor_lang::system_program::Transfer {
            from: self.owner.to_account_info(),
            to: self.escrow_account.to_account_info(),
        };

        let cpi_ctx_sol = CpiContext::new(
            self.system_program.to_account_info(),
            transfer_sol_instruction
        );
        anchor_lang::system_program::transfer(cpi_ctx_sol, LAMPORT_PER_SOL)?;

        Ok(())
    }
}