use anchor_lang::prelude::*;

declare_id("");

#[program]
pub mod insecure_signer_authorization {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        msg!("Hello {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

// @audit-issue :: Here there is no signer check for authorization
#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: AccountInfo<'info>,
}


// @follow-up There are two ways to do it 
// 1. Just update the program
pub mod insecure_signer_authorization {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        msg!("Hello {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

// 2. Make a check in the derived accounts
#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: Signer<'info>
}

