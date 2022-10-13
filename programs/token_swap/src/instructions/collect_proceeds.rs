use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, self};

use crate::Registry;

#[derive(Accounts)]
pub struct CollectProceeds<'info> {
    #[account(has_one = admin)]
    pub registry: Account<'info, Registry>,
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault_token_in".as_ref(), registry.key().as_ref()],
        bump
    )]
    pub vault_token_in: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = admin_proceeds_account.owner == admin.key()
    )]
    pub admin_proceeds_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}


pub fn collect_proceeds(ctx: Context<CollectProceeds>) -> Result<()> {
    let registry_key = ctx.accounts.registry.key();

    let (_, nonce) = Pubkey::find_program_address(
        &[b"vault_token_in".as_ref(), registry_key.as_ref()],
        ctx.program_id,
    );
    let seeds = &[b"vault_token_in".as_ref(), registry_key.as_ref(), &[nonce]];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.vault_token_in.to_account_info(),
                to: ctx.accounts.admin_proceeds_account.to_account_info(),
                authority: ctx.accounts.vault_token_in.to_account_info(),
            },
            &[seeds],
        ),
        ctx.accounts.vault_token_in.amount,
    )?;

    Ok(())
}
