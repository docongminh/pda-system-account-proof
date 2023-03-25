use anchor_lang::prelude::*;

declare_id!("DhNTR8sYeKHLraVBsDGx6RMkBTwCYQ4v3ZWLhd9pmNWu");

#[program]
pub mod hacker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
