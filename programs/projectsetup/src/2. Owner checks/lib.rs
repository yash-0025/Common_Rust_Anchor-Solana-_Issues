use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;

declare_id!("");
#[program]
pub mod insecure_owner_check {
    use super::*;


    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        if ctx.accounts.authority.key() != &token.owner {
            return Err(ProgramError::InvalidAccountData);
        }
        msg!("Account Balance is {}", token.amount);
        Ok(())
    }
}
// Missing check which validates proper SPL TOken Account is passed or not 
#[derive(Accounts)]
pub struct LogMessage<'info> {
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}

// Two ways to solve this issue and apply the owner checks

//*// 1. We can update the derive(accounts) and add a constraint here it automaticaly verifies that its a real token account

#[derive(Accounts)]
pub struct LogMessage<'info> {
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}

//*//  2. We can manually check in the function that is this a genuine spl token account and does this account belongs to this caller

pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
    let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow());
    if ctx.accounts.token.owner != &spl_token::ID {
        return Err(ProgramError::InvalidAccountData);
    }
    if ctx.accounts.authority.key != &token.owner {
        return Err(ProgramError::InvalidAccountData);
    }
    msg!("Balance is :: {}", token.account);
    Ok(())
}