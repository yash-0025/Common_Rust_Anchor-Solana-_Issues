use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id("");


#[program]
pub mod arbitrarty_cpi {
    use super::*;

// @audit-issue :: There is no check for the Cpi inputs 
//^//   Attacker can pass fake token program which can steal funds as this code accepts any program. 
//^//   Fake Token accounts can be passed as source and destination
//^//   Authority is not verified there must be a signer check

    pub fn cpi(ctx: Context<Cpi>, amount: u64) -> Result<()> { 
        solana_program::program::invoke(
            &spl_token::instruction::transfer(
                ctx.accounts.token_progrm_key,
                ctx.accounts.source.key,
                ctx.accounts.destination.key,
                ctx.accounts.authority.key,
                &[],
                amount,
            )?,
            &[
                ctx.accounts.souce.clone(),
                ctx.accounts.destination.clone(),
                ctx.accounts.authority.clone(),
            ]
        )

    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    source: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}


//*// 1.] One way to do is implementing poper accounts using the Anchor ways and adding a type-safe context for cpi whcih anchor can validate and it ensures all the accoutns meet SPL toke requirement before execution

pub fn cpi(ctx: Context<Cpi>, amount: u64) ->Result<()> {
    token::transfer(ctx.accounts.transfer_ctx(), amount)
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    source: Account<'info, TokenAccount>,
    destination: Account<'info, TokenAccount>,
    authority: Signer<'info>,
    token_program:Program<'info, Token>,
}

impl<'info> Cpi<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_,'_,'_,'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Tranfer {
            from: self.source.to_account_info(),
            to: self.destination.to_account_info()
            to: self.destination.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}


//*// 2.] We can manually secured the CPI pattern that fixes the arbitrary cpi issue. We will add a check which will verify that the token program is the official SPL Token program

pub fn cpi(ctx: Context<Cpi>, amount:u64) -> Result {

    if &spl_token::ID != ctx.accounts.token_program.key {
        return Err(ProgramErr::IncorrectProgramId);
    }
    solana_program::program::invoke(
        &spl_token::instruction::transfer(
            ctx.accounts.token_progrm_key,
            ctx.accounts.source.key,
            ctx.accounts.destination.key,
            ctx.accounts.authority.key,
            &[],
            amount,
        )?,
        &[
            ctx.accounts.souce.clone(),
            ctx.accounts.destination.clone(),
            ctx.accounts.authority.clone(),
        ]
    )
}