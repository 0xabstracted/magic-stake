use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, self};

use crate::{Registry, error::DispenserError};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(has_one = vault_token_in, has_one = vault_token_out)]
    registry: Account<'info, Registry>,
    swapper: Signer<'info>,
    #[account(mut)]
    vault_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    vault_token_out: Account<'info, TokenAccount>,
    #[account(mut)]
    buyer_token_in_account: Account<'info, TokenAccount>,
    #[account(mut)]
    buyer_token_out_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}

pub fn swap(ctx: Context<Swap>, amount_requested: u64) -> Result<()> {
    if ctx.accounts.vault_token_out.amount < amount_requested {
        return Err(error!(DispenserError::InsufficientVaultFunds));
    }

    // `checked_div` will truncate any potential remainder
    // consequence is to slightly under-charge the user by the remainder amount
    let amount_token_in = amount_requested
        .checked_mul(ctx.accounts.registry.rate_token_in)
        .ok_or(DispenserError::InvalidCalculation)?
        .checked_div(ctx.accounts.registry.rate_token_out)
        .ok_or(DispenserError::InvalidCalculation)?;

    if ctx.accounts.buyer_token_in_account.amount < amount_token_in {
        return Err(DispenserError::InsufficientUserFunds.into());
    }

    msg!("Amount requested: {}", amount_requested);
    msg!("Amount charged: {}", amount_token_in);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.buyer_token_in_account.to_account_info(),
                to: ctx.accounts.vault_token_in.to_account_info(),
                authority: ctx.accounts.swapper.to_account_info(),
            },
        ),
        amount_token_in,
    )?;

    let registry_key = ctx.accounts.registry.key();
    let (_, nonce) = Pubkey::find_program_address(
        &[b"vault_token_out".as_ref(), registry_key.as_ref()],
        ctx.program_id,
    );
    let seeds = &[b"vault_token_out".as_ref(), registry_key.as_ref(), &[nonce]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.vault_token_out.to_account_info(),
                to: ctx.accounts.buyer_token_out_account.to_account_info(),
                authority: ctx.accounts.vault_token_out.to_account_info(),
            },
            &[seeds],
        ),
        amount_requested,
    )?;

    Ok(())
}
