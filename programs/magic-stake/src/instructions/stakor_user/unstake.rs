use anchor_lang::prelude::*;
use anchor_spl::{token::{self, Token, Transfer, TokenAccount, Mint}, associated_token::AssociatedToken};

use gem_bank::{
    self,
    cpi::accounts::SetVaultLock,
    cpi::accounts::WithdrawGem,
    program::GemBank,
    state::{Bank, Vault},
};
use gem_common::{now_ts, TrySub, TryDiv, TryMul, close_account};
use crate::state::{Farm, FarmerState, Farmer, FarmerStakedMints};



#[derive(Accounts)]
#[instruction(bump_auth: u8, bump_token_treasury: u8, bump_farmer: u8, index: u32)]
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
    #[account(
        mut,
        seeds = [
            b"farmer_staked_mints".as_ref(), 
            // &index.to_le_bytes(),
            farmer.key().as_ref(),
        ],
        bump = farmer_staked_mints.load()?.bump,
        has_one = farmer,
    )]
    // #[account(mut)]
    pub farmer_staked_mints: AccountLoader<'info, FarmerStakedMints>,
    #[account(mut)]
    pub identity: Signer<'info>,

    // cpi
    #[account(constraint = bank.bank_manager == farm_authority.key())]
    pub bank: Box<Account<'info, Bank>>,
    #[account(mut)]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK:
    pub vault_authority: AccountInfo<'info>,
    // trying to deserialize here leads to errors (doesn't exist yet)
    /// CHECK:
    #[account(mut)]
    pub gem_box: AccountInfo<'info>,
    // trying to deserialize here leads to errors (doesn't exist yet)
    /// CHECK:
    #[account(mut)]
    pub gem_deposit_receipt: AccountInfo<'info>,
    #[account(mut)]
    pub gem_destination: Box<Account<'info, TokenAccount>>,
    pub gem_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    pub gem_rarity: AccountInfo<'info>,
    pub gem_bank: Program<'info, GemBank>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
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
    fn withdraw_gem_ctx(&self) -> CpiContext<'_, '_, '_, 'info, WithdrawGem<'info>> {
        CpiContext::new(
            self.gem_bank.to_account_info(),
            WithdrawGem {
                bank: self.bank.to_account_info(),
                vault: self.vault.to_account_info(),
                owner: self.identity.to_account_info(),
                authority: self.vault_authority.clone(),
                gem_box: self.gem_box.clone(),
                gem_deposit_receipt: self.gem_deposit_receipt.clone(),
                gem_destination: self.gem_destination.to_account_info(),
                gem_mint: self.gem_mint.to_account_info(),
                gem_rarity: self.gem_rarity.clone(),
                receiver: self.identity.to_account_info(),
                token_program: self.token_program.to_account_info(),
                associated_token_program: self.associated_token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
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

pub fn handler(ctx: Context<Unstake>, 
        skip_rewards: bool , 
        bump_auth: u8, 
        bump_gem_box: u8, 
        bump_gdr:u8, 
        bump_rarity: u8, 
        amount: u64,
        index: u32,
    ) -> Result<()> {
    // collect any unstaking fee
     
    let farm = &mut ctx.accounts.farm;
    let farmer = &mut ctx.accounts.farmer;
    let now_ts = now_ts()?;

    // skipping rewards is an EMERGENCY measure in case farmer's rewards are overflowing
    // at least this lets them get their assets out
    if !skip_rewards {
        farm.update_rewards(now_ts, Some(farmer), false)?;
        // farm.update_lp_points(now_ts, Some(farmer), false)?;
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
        gem_bank::cpi::withdraw_gem(
            ctx.accounts.withdraw_gem_ctx(),
            bump_auth, 
            bump_gem_box, 
            bump_gdr, 
            bump_rarity, 
            amount,
        )?;
    }
    let mut farmer_staked_mints = ctx.accounts.farmer_staked_mints.load_mut()?;
    if farmer_staked_mints.index == index {
        for _ in 0..amount{
            farmer_staked_mints.remove_nft(ctx.accounts.gem_mint.key())?;
        }
    }
    // if farmer_staked_mints.no_of_nfts_staked == 0 {
    //     close_account(farmer_staked_mints, &mut ctx.accounts.identity.to_account_info())?;
    // }
   
    Ok(())
}