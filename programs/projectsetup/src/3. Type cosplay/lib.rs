use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("");

#[program]
pub mod type_cosplay_insecure {
    use super::*;

// @audit-issue ::There is no check that the account which is going to be passed is an user account and attacker can pass any account which matches its type
//?//  As there are two accounts Normal AccountInfo accounts and Metadata accounts so attacker can use the Metadata account and it will pass that pubkey.
    pub fn update_user(ctx: Context<UpdateUser>) -> ProgramResult {
        let user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        if ctx.accounts.user.owner != ctx.program_id {
            return Err(ProgramError::IllegalOwner);
        }
        if user.authority != ctx.accounts.authority.key() {
            return Err(ProgramError::InvalidAccountData);
        }

        msg!("Hello {}", user.authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    authority:Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Metadata {
    account: Pubkey,
}

//^//  Two ways to solve this 

//*//  1.] By adding an anchor check using has_one = authority and making it a UserAccount from AccountInfo account
//!//  It checks that the `user.authority == athority.key`


#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(has_one = authority)]
    user:Account<'info, User>,
    authority: Signer<'info>,
}


//*// 2.] Other way to do is to add a manual check in the function logic using enum
pub fn update_user(ctx: Context<UpdateUser>) ->ProgramResult {
    let user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
    if ctx.accounts.user.owner != ctx.program_id {
        return Err(ProgramError::IllegalOwner);
    }
    if user.authority != ctx.accounts.authority.key() {
        return Err(ProgramError::InvalidAccountData);
    }
    if user.discriminant != AccountDiscriminant::User {
        return Err(ProgramError::InvalidAccountData);
    }
    msg!("Hello {}", user.authority);
    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AccountDiscriminant {
    User,
    Metadata
}