use anchor_lang::prelude::*;

declare_id("");

#[program]
pub mod duplicate_mutable_accounts {
    use super::*;

//^//  @audit-issue :: Both user a and user b can be same account . So attakcer could pass same account twice and bypass the logic expecting two distinct accounts which can leads to data corruption

    pub fn update(ctx:Context<Update>, a: u64, b: u64,) ->Result<()> {
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        user_a.data = a;
        user_b.data = b;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Update<'info> {
    user_a: Account<'info,User>,
    user_b:Account<'info, User>, 
}

#[account]
pub struct User {
    data: u64,
}

//*//  1. One way to solve this issue is by adding constraint which will check that both address is not same 
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(constraint = user_a.key() != user_b.key())]
    user_a: Account<'info, User>,
    user_b: Account<'info, User>,
}

//*// 2. Another way is to add a manual check in the function 

pub fn update(ctx:Context<Update>, a: u64, b: u64,) ->Result<()> {
    if ctx.accounts.user_a.key() == ctx.accounts.user_b.key() {
        return Err(ProgramError::InvalidArgument);
    }
    
    let user_a = &mut ctx.accounts.user_a;
    let user_b = &mut ctx.accounts.user_b;

    user_a.data = a;
    user_b.data = b;
    Ok(())
}