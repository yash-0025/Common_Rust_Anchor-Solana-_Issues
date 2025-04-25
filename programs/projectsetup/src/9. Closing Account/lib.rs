use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod closing_accounts_insecure {
    use super::*;

//^// @audit-issue  ::  Here the account is owner  check which is getting closed so attacker can pass any account address and steal the lamports. There is no signer check for destination also

    pub fn close(ctx: Context<Close>) -> Result {
        let dest_starting_lamports = ctx.accounts.destination.lamports();

        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports.checked_add(ctx.accounts.account.to_account_info().lamports);
        **ctx.accounts.account.to_account_info().lamports.borrow_mut() = 0;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Close<'info> {
    account: Account<'info, Data>,
    destination: AccountInfo<'info>,
}

#[account]
pub struct Data{
    data: u64,
}


//*// 1. One way to solve this issue is adding checks in the Accounts
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = destination)]
    account: Account<'info, Data>,
    #[account(mut)]
    destination: AccountInfo<'info>,
}

