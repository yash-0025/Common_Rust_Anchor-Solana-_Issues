use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("");


#[program]
pub mod pda_sharing {
    use super::*;

//^//  @audit-issue :: Tokenpool PDA is derived from the mint address  which means all pool for the same token mint will share same PDA and Attacker could create fake TokenPool with same mint and drain fund fomr other pool using the same mint as it will bypass the authorization checks

    pub fn withdraw_token(ctx: Context<WithdrawTokens>) -> Result<()> {
        let amount = ctx.accounts.vault.amount;
        let seeds = &[ctx.accounts.pool.mint.as_ref(), &[ctx.accounts.pool.bump]];
        token::transfer(Ctx.accounts.transfer_ctx().with_signer(&[seeds]),amount)
    }
}

#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(
        has_one = vault,
        has_one = withdraw_destination
    )]
    pool: Account<'info, TokenPool>,
    vault: Account<'info, TokenAccount>,
    withdraw_destination: Account<'info, TokenAccount>,
    authority: Signer<'info>,
    token_program: Program<'info, Token>,
}

impl<'info> WithdrawTokens<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_,'_,'_,'info, token::Tranfer<'info>>{
        let program = self.token_program.to_account_info();
        let accounts = token::Tranfer {
            from :self.vault.to_account_info(),
            to: self.withdraw_destination.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

#[account]
pub struct TokenPool {
    vault: Pubkey,
    mint: Pubkey,
    withdraw_destination: Pubkey,
    bump: u8,
}

//*// 1. One way to  solve this issue is using withdraw_destination means the destination address as a seed to make it unique so it will prevent different pools form sharing the same authority

pub fn withdraw_token(ctx: Context<WithdrawTokens>) -> Result<()> {
    let amount = ctx.accounts.vault.amount;
    let seeds = &[
        ctx.accounts.pool.withdraw_destination.as_ref(), &[ctx.accounts.pool.bump],
    ];
    token::transfer(ctx.accounts.transfer_ctx().with_signer(&[seeds]), amount)
}

#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(
				has_one = vault,
				has_one = withdraw_destination,
				seeds = [withdraw_destination.key().as_ref()],
				bump = pool.bump,
		)]
    pool: Account<'info, TokenPool>,
    vault: Account<'info, TokenAccount>,
    withdraw_destination: Account<'info, TokenAccount>,
    authority: Signer<'info>,
    token_program: Program<'info, Token>,
}


