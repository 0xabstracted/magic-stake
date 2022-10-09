pub use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, self, Token};
use gem_common::errors::ErrorCode;

pub use crate::state::AlphaTokenswap;

#[derive(Accounts)]
#[instruction(bump_alpha_pot: u8)]
pub struct TransferAlphaTokens <'info> {
    #[account(
        mut, has_one = alpha_creator
    )]
    pub alpha_tokenswap: Account<'info, AlphaTokenswap>,
    /// CHECK:
    pub alpha_creator: AccountInfo<'info>,
    #[account(
        mut, seeds = [b"alpha_pot".as_ref(), alpha_tokenswap.key().as_ref()], bump = bump_alpha_pot,
    )]
    pub alpha_pot: Account<'info, TokenAccount>,
    pub alpha_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferAlphaTokens>, amount: u64) -> Result<()>{
    let alpha_tokenswap_account_info = ctx.accounts.alpha_tokenswap.to_account_info();
    let alpha_creator = &ctx.accounts.alpha_creator;
    if ctx.accounts.alpha_pot.amount < amount {
        return Err(ErrorCode::EmptySwapPot.into());
    }

    let (_, nonce) = Pubkey::find_program_address(
        &[b"alpha_tokenswap".as_ref(), alpha_creator.key.as_ref()],
        ctx.program_id,
    );
    let seeds = &[b"alpha_tokenswap".as_ref(), alpha_creator.key.as_ref(), &[nonce]];
    let signer_seeds = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info().clone(),
            token::Transfer {
                from: ctx.accounts.alpha_pot.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: alpha_tokenswap_account_info,
            },
            signer_seeds,
        ),
        amount,
    )?;
    Ok(())
}