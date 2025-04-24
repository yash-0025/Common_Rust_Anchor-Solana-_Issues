use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id("");


#[program]
pub mod arbitrarty_cpi {
    use super::*;

    pub fn cpi(ctx: Context<Cpi>, amount: u64) -> Result<()> {

    }
}

#[derive(Accounts)]
pub struct Cpi<'info>