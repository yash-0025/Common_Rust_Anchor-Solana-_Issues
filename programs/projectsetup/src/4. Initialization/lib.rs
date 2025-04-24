use anchor_lang::prelude::*;
use borsh:;{BorshSerialize, BorshDeserialize};
use std::ops::DerefMut;

declare_id!("");

// @audit-issue :: NO check to prevent multiple initialize. Missing check for preinitialized account. Attacker can pass fake user account where authority is the attacker keys and pass it inside the initialize and can now hold the authority of that accoutn

#[program]
pub mod initialization{
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        user.authority = ctx.accounts.authority.key();
        let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
        user.serialize(storage.deref_mut()).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    authority: Pubkey,
}



//*// 1.] We can use anchor prebuild constraints like init : which checks account is newly created , allocate space automatically and sets the program as owner and also we can use User account instead of AccountInfo it will only allows correct account type to be passed which has proper ownership an valid data structure

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8+32
    )]
    user:Account<'info, User>,
    #[account(mut)]
    authority:Signer<'info>,
    system_program: Program<'info, System>,
}


//~// 2.] Other way is we can add a manual check which prevents all this. By adding discriminator it will allow only once to initialize and after that the disciminator value will be set to true so it will not allow to initialize again
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let mut user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
    if !user.discriminator {
        return Err(ProgramError::InvalidAccountData)
    }
    user.authority = ctx.accounts.authority.key(),
    user.disciminator = true;

    let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
    user.serialize(storage.deref_mut()).unwrap();

    msg!("Hello");
    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    discriminator: bool,
    authority: Pubkey,
}
