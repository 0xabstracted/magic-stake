use crate::state::Farm;
use crate::state::Farmer;
use anchor_lang::prelude::*;
use gem_bank::{
    self,
    cpi::accounts::SetVaultLock,
    program::GemBank,
    state::{Bank, Vault},
};
use gem_common::now_ts;
use gem_common::errors::ErrorCode;


#[derive(Accounts)]
#[instruction(bump_auth: u8, bump_farmer: u8)]
pub struct StakeAlpha<'info> {
    // farm
    #[account(mut, has_one = farm_authority, has_one = bank)]
    pub farm: Box<Account<'info, Farm>>,
    /// CHECK:
    #[account(seeds = [farm.key().as_ref()], bump = bump_auth)]
    pub farm_authority: AccountInfo<'info>,

    // farmer
    #[account(mut, has_one = farm, has_one = identity, has_one = vault,
        seeds = [
            b"farmer".as_ref(),
            farm.key().as_ref(),
            identity.key().as_ref(),
        ],
        bump = bump_farmer)]
    pub farmer: Box<Account<'info, Farmer>>,
    #[account(mut)]
    pub identity: Signer<'info>,

    // cpi
    #[account(constraint = bank.bank_manager == farm_authority.key())]
    pub bank: Box<Account<'info, Bank>>,
    #[account(mut)]
    pub vault: Box<Account<'info, Vault>>,
    pub gem_bank: Program<'info, GemBank>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakeAlpha<'info> {
    fn set_lock_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetVaultLock<'info>> {
        CpiContext::new(
            self.gem_bank.to_account_info(),
            SetVaultLock {
                bank: self.bank.to_account_info(),
                vault: self.vault.to_account_info(),
                bank_manager: self.farm_authority.clone(),
            },
        )
    }
}

pub fn handler(ctx: Context<StakeAlpha>) -> Result<()> {
    if ctx.accounts.vault.gem_count == 0 {
        return Err(error!(ErrorCode::VaultIsEmpty));
    }

    // lock the vault so the user can't withdraw their gems
    gem_bank::cpi::set_vault_lock(
        ctx.accounts
            .set_lock_vault_ctx()
            .with_signer(&[&ctx.accounts.farm.farm_seeds()]),
        true,
    )?;

    // update accrued rewards BEFORE we increment the stake
    let farm = &mut ctx.accounts.farm;
    let farmer = &mut ctx.accounts.farmer;
    let vault = &ctx.accounts.vault;
    let now_ts = now_ts()?;
    farm.update_rewards_alpha(now_ts, Some(farmer), true)?;
    farm.begin_staking_alpha(now_ts, vault.gem_count, vault.rarity_points, farmer)?;

    //collect a fee for staking
    // ctx.accounts.transfer_fee()?;

    // msg!("{} gems staked by {}", farmer.gems_staked, farmer.key());
    Ok(())
}