use anchor_lang::prelude::*;

use crate::Registry;

#[derive(Accounts)]
pub struct UpdateRegistry<'info> {
    #[account(mut, has_one = admin)]
    registry: Account<'info, Registry>,
    admin: Signer<'info>,
}

pub fn update_registry(
    ctx: Context<UpdateRegistry>,
    rate_token_in: u64,
    rate_token_out: u64,
) -> Result<()> {
    ctx.accounts.registry.admin = ctx.accounts.admin.key();
    ctx.accounts.registry.rate_token_in = rate_token_in;
    ctx.accounts.registry.rate_token_out = rate_token_out;

    Ok(())
}
