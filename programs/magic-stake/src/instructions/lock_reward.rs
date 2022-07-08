use crate::state::Farm;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct LockReward<'info> {
    #[account(mut, has_one = farm_manager)]
    pub farm: Box<Account<'info, Farm>>,
    #[account(mut)]
    pub farm_manager: Signer<'info>,
    pub reward_mint: Box<Account<'info, Mint>>,
}

pub fn handler(ctx: Context<LockReward>) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    farm.lock_reward_by_mint(ctx.accounts.reward_mint.key())?;
    Ok(())
}
