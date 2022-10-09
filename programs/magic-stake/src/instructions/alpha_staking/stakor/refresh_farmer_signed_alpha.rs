use anchor_lang::prelude::*;
use gem_common::now_ts;

use crate::state::Farm;
use crate::state::Farmer;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct RefreshFarmerSignedAlpha<'info> {
    // farm
    #[account(mut)]
    pub farm: Box<Account<'info, Farm>>,

    // farmer
    #[account(mut, has_one = farm, has_one = identity, seeds = [
            b"farmer".as_ref(),
            farm.key().as_ref(),
            identity.key().as_ref(),
        ],
        bump = bump)]
    pub farmer: Box<Account<'info, Farmer>>,
    pub identity: Signer<'info>, // <--- the diff
}

pub fn handler(ctx: Context<RefreshFarmerSignedAlpha>, reenroll: bool) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    let farmer = &mut ctx.accounts.farmer;
    let now_ts = now_ts()?;

    farm.update_rewards_alpha(now_ts, Some(farmer), reenroll)?;
    msg!("{} farmer refreshed (SIGNED)", farmer.key());
    Ok(())
}