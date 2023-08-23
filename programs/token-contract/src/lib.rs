use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{InitializeMint, MintTo, Token, Transfer};

declare_id!("BzVRmDPEmh9yLb9hC1CXGC9cNuwMM57bR3Dn2rxyPTfo");

#[program]
pub mod token_contract {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        //Create MintTo struct for our context
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute the anchor helper function to mint tokens
        token::mint_to(cpi_ctx, 10)?;

        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        //Create transfer struct for our context
        let transfer_instruction = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the Context we need for the  transfer request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        // Execute the anchor helper function to transfer tokens
        anchor_spl::token::transfer(cpi_ctx, 5)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: THis is not dangerous because we don't need to read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: THis is not dangerous because we don't need to read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: THis is not dangerous because we don't need to read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: THis is not dangerous because we don't need to read or write from this account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: THis is not dangerous because we don't need to read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
