use anchor_lang::prelude::*;
use anchor_spl:: {
    associated_token:: AssociatedToken,
    token2022::{
        initialize_mint_close_authority, initialize_non_transferable_mint, spl_tokne_2022,
    },
    token_interface::{
        spl_token_metadata_interface::state::Field, Mint, TokenAccount, TokenInterface,
    },
};

use mpl_tokne_metadata::{
    instructions::{
        CreateV1Builder, MintV1Builder, SetAndVerifyCollectionV1Builder, UpdateV1Builder,
    },
    type::DataV2,
};

declare_id!("Ep6Cgue7MrTCpTqFhCZfF9yMPA8VA2CJg1u5pmuJ6yQ9");


#[program]
mod myanchorproject {
    use super::*;

    // Initialize a new token with metadata
    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        name: String,
        symbol: String,
        uri: String,
        decimals: u8,
        enable_non_transferable: bool;
    ) -> Result<()> {
        let mint =&ctx.accounts.mint;
        let metadata = &ctx.accounts.metadata;
        let payer = &ctx.accounts.payer;
        let token_program = &ctx.accounts.token_program;

        // Initialize mint with Token2022 exensions if needed
        if enable_non_transferable{
            let cpi_accounts = spl_token_2022::instruction::InitializeNonTransferableMint {
                mint: mint.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(
                token_program.to_account_info(),
                cpi_accounts,
            );
            initialize_non_transferable_mint(cpi_ctx)?;
        } else {
            let cpi_accounts = spl_token_2022::instruction::InitializeTransferableMint{
                mint: mint.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(
                token_program.to_account_info(),
                cpi_accounts,
            );
            initialize_mint_close_authority(
                cpi_ctx,
                Some(ctx.accounts.payer.key()),
                decimals,
            )?;
        }

        // Create Metadata using MPL
        let create_metadata_ix = CreateV1Builder::new()
            .metadata(metadata.key())
            .mint(mint.key())
            .authority(payer.key())
            .payer(payer.key())
            .update_authority(payer.key())
            .system_program(ctx.accounts.system_program.to_account_info().key())
            .data(DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            })
            .instruction();
            anchor_lang::solana_program::program::invoke(
                &create_metadata_ix,
                &[
                    metadata.to_account_info(),
                    mint.to_account_info(),
                    payer.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
            Ok(())
    }

    // Mint Token to a user
    pub fn mint_to_user(
        ctx: Context<MintToUser>,
        amount: u64,
    ) -> Result <()> {
        let cpi_accounts = spl_token_2022::instruction::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        anchor_spl::token_2022::mint_to(cpi_ctx, amount)?;

        // Update metadata if needed
        let update_metadata_ix = UpdateV1Builder::new()
            .metadata(ctx.accounts.metadata.key())
            .update_authority(ctx.accounts.payer.key())
            .new_update_authority(Some(ctx.accounts.payer.key()))
            .primary_sale_happened(Some(true))
            .instruction();

        anchor_lang::solana_program::program::invoke(
            &update_metadata_ix,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.payer.to_account_info(),
            ],
        )?;
        Ok(())
    }

    // Transfer tokens between users
    pub fn transfer_tokens(
        ctx: Context<TranferToken>,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts = spl_token_2022::instruction::Tranfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        anchor_spl::token2022::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
// @follow-up What are this InterfaceAccount and Mint deserialization
    #[account(
        init,
        payer = payer,
        mint::token_program token_program,
        mint::decimals = 9,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


// Accounts for minting token to user
#[derive(Accounts)]
pub struct MintToUser<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub user: UncheckedAccount<'info>,
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TranferToken<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub from_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, TokenInterface>,
}