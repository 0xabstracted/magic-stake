use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};

use crate::Registry;

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(
        init,
        seeds = ["registry".as_ref(), admin.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Registry>(),
        payer = admin,
    )]
    registry: Account<'info, Registry>,
    #[account(
        init,
        seeds = [b"vault_token_in".as_ref(), registry.key().as_ref()],
        bump,
        payer = admin,
        token::authority = vault_token_in,
        token::mint = mint_token_in,
    )]
    vault_token_in: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [b"vault_token_out".as_ref(), registry.key().as_ref()],
        bump,
        payer = admin,
        token::authority = vault_token_out,
        token::mint = mint_token_out,
    )]
    vault_token_out: Account<'info, TokenAccount>,
    #[account(mut)]
    admin: Signer<'info>,
    mint_token_in: Account<'info, Mint>,
    mint_token_out: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn create_registry(
    ctx: Context<CreateRegistry>,
    rate_token_in: u64,
    rate_token_out: u64,
) -> Result<()> {
    ctx.accounts.registry.vault_token_in = ctx.accounts.vault_token_in.key();
    ctx.accounts.registry.vault_token_out = ctx.accounts.vault_token_out.key();
    ctx.accounts.registry.admin = ctx.accounts.admin.key();
    ctx.accounts.registry.rate_token_in = rate_token_in;
    ctx.accounts.registry.rate_token_out = rate_token_out;
    ctx.accounts.registry.mint_token_in = ctx.accounts.mint_token_in.key();
    ctx.accounts.registry.mint_token_out = ctx.accounts.mint_token_out.key();

    Ok(())
}
