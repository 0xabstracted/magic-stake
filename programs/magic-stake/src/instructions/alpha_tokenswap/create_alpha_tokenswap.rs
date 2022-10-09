pub use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token, self};

use crate::state::alpha_token_swaps::AlphaTokenswap;

#[derive(Accounts)]
pub struct CreateAlphaTokenswap <'info> {
    #[account(
        init,
        seeds = [b"alpha_tokenswap".as_ref(), alpha_creator.key().as_ref(), alpha_mint.key().as_ref(),],
        bump,
        payer = alpha_creator,
        space = 8 + std::mem::size_of::<AlphaTokenswap>(), 
    )]
    pub alpha_tokenswap: Account<'info, AlphaTokenswap>,
    #[account(mut)]
    pub alpha_creator: Signer<'info>,
    #[account(
        init,
        seeds = [b"alpha_pot".as_ref(), alpha_tokenswap.key().as_ref(), alpha_mint.key().as_ref()],
        bump,
        payer = alpha_creator,
        token::mint = alpha_mint,
        token::authority = alpha_tokenswap,
    )]
    pub alpha_pot: Account<'info, TokenAccount>,
    #[account(mut)]
    pub alpha_owner_source: Account<'info, TokenAccount>,
    pub alpha_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateAlphaTokenswap>, amount: u64) -> Result<()> {
    let alpha_tokenswap = &mut ctx.accounts.alpha_tokenswap;
    alpha_tokenswap.alpha_creator = *ctx.accounts.alpha_creator.key;
    alpha_tokenswap.alpha_mint = ctx.accounts.alpha_mint.key();
    alpha_tokenswap.alpha_pot = ctx.accounts.alpha_pot.key();
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.alpha_owner_source.to_account_info(),
                to: ctx.accounts.alpha_pot.to_account_info(),
                authority: ctx.accounts.alpha_creator.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}