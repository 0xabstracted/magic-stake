use anchor_lang::{
    prelude::*,
};
use anchor_spl::token::{self, Token, Transfer, TokenAccount};

use gem_bank::{
    self,
    cpi::accounts::SetVaultLock,
    program::GemBank,
    state::{Bank, Vault},
};
use gem_common::{now_ts, TrySub, TryDiv, TryMul};
use crate::state::{Farm, FarmerState};

use crate::state::Farmer;


#[derive(Accounts)]
#[instruction(bump_auth: u8, bump_token_treasury: u8, bump_farmer: u8)]
pub struct Unstake<'info> {
    // farm
    #[account(mut, has_one = farm_authority, has_one = farm_treasury_token, has_one = bank)]
    pub farm: Box<Account<'info, Farm>>,
    /// CHECK:
    #[account(seeds = [farm.key().as_ref()], bump = bump_auth)]
    pub farm_authority: AccountInfo<'info>,
    #[account( seeds = [
        b"token_treasury".as_ref(),
        farm.key().as_ref(),
    ],
    bump = bump_token_treasury,
    )]
    pub farm_treasury_token: Box<Account<'info, TokenAccount>>,

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
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
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
    fn pay_tokens_treasury_ctx(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(), 
            Transfer { 
                from: self.identity.to_account_info(), 
                to: self.farm_treasury_token.to_account_info(), 
                authority: self.identity.to_account_info(), 
            },
        )
    }
}

pub fn handler(ctx: Context<Unstake>, skip_rewards: bool) -> Result<()> {
    // collect any unstaking fee
     
    let farm = &mut ctx.accounts.farm;
    let farmer = &mut ctx.accounts.farmer;
    let now_ts = now_ts()?;

    // skipping rewards is an EMERGENCY measure in case farmer's rewards are overflowing
    // at least this lets them get their assets out
    if !skip_rewards {
        farm.update_rewards(now_ts, Some(farmer), false)?;
        farm.update_lp_points(now_ts, Some(farmer), false)?;
    }
    // end staking (will cycle through state on repeated calls)
    farm.end_staking(now_ts, farmer)?;
    let farm = &ctx.accounts.farm;
    let farmer = &ctx.accounts.farmer;

    if farmer.state == FarmerState::Unstaked && farm.config.unstaking_fee_percent > 0 && farm.config.unstaking_fee_percent < 100 {
       let unstake_fee_tokens = farmer.reward_a.accrued_reward.try_mul(farm.config.unstaking_fee_percent.try_div(100)?)?;
       
       token::transfer(
            ctx.accounts
            .pay_tokens_treasury_ctx()
            .with_signer(&[&ctx.accounts.farm.farm_seeds()]),
            unstake_fee_tokens,
        )?;
        let farmer = &mut ctx.accounts.farmer;
        farmer.reward_a.accrued_reward.try_sub_assign(unstake_fee_tokens)?; 
    
    }
    let farmer = &ctx.accounts.farmer;
      
    if farmer.state == FarmerState::Unstaked {
        // unlock the vault so the user can withdraw their gems
        gem_bank::cpi::set_vault_lock(
            ctx.accounts
                .set_lock_vault_ctx()
                .with_signer(&[&ctx.accounts.farm.farm_seeds()]),
            false,
        )?;
    }
    
    Ok(())
}