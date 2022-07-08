use anchor_lang::{
    prelude::*, 
    // solana_program::{
    //     program::invoke,
    //     system_instruction
    // }
};
use anchor_spl::token::{self, Token, Transfer};
use gem_bank::{self,
    program::GemBank,
    cpi::accounts::SetVaultLock,
    state::{Bank, Vault},
};
use gem_common::{now_ts, TryDiv};
use crate::state::{Farm, FarmerState};
use crate::state::Farmer;

#[derive(Accounts)]
#[instruction(bump_auth: u8, bump_treasury: u8, bump_farmer: u8)]
pub struct Unstake<'info> {
    #[account(mut, has_one = farm_authority, has_one = farm_treasury, has_one = bank)]
    pub farm: Box<Account<'info, Farm>>,
    ///CHECK:
    #[account(seeds = [farm.key().as_ref()], bump = bump_auth)]
    pub farm_authority: AccountInfo<'info>,
    ///CHECK:
    #[account(mut, seeds = [b"treasury".as_ref(), farm.key().as_ref()], bump = bump_treasury)]
    pub farm_treasury: AccountInfo<'info>,
    #[account(mut, has_one = farm, has_one = identity, has_one = vault, 
            seeds = [b"farmer".as_ref(), farm.key().as_ref(), identity.key().as_ref()],
            bump = bump_farmer )]
    pub farmer: Box<Account<'info, Farmer>>,
    #[account(mut)]
    pub identity: Signer<'info>,
    #[account(constraint = bank.bank_manager == farm_authority.key())]
    pub bank: Box<Account<'info, Bank>>,
    #[account(mut)]
    pub vault: Box<Account<'info, Vault>>,
    pub gem_bank: Program<'info, GemBank>,
    // ///CHECK:
    // #[account(mut, address = Pubkey::from_str(FEE_WALLET).unwrap())]
    // pub fee_acc: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake <'info>{
    fn set_lock_vault_ctx(&self) -> CpiContext<'_,'_,'_, 'info,SetVaultLock<'info>> {
        CpiContext::new(
            self.gem_bank.to_account_info(),
            SetVaultLock { 
                bank: self.bank.to_account_info(), 
                bank_manager: self.farm_authority.clone(), 
                vault: self.vault.to_account_info()
            },
        )
    }

    // fn _pay_treasury(&self, lamports: u64) -> Result<()> {
    //     invoke(
    //         &system_instruction::transfer(self.identity.key, self.farm_treasury.key, lamports),
    //         &[
    //             self.identity.to_account_info(),
    //             self.farm_treasury.clone(),
    //             self.system_program.to_account_info(),
    //         ],
    //     ).map_err(Into::into)
    // } 

    fn pay_tokens_treasury_ctx(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(), 
            Transfer { 
                from: self.identity.to_account_info(), 
                to: self.farm_treasury.clone(), 
                authority: self.identity.to_account_info(), 
            },
        )
    }
    
    // fn _transfer_fee(&self, lamports: u64) -> Result<()> {
    //     invoke(&system_instruction::transfer(self.identity.key, self.fee_acc.key, lamports),
    //      &[
    //         self.identity.to_account_info(),
    //         self.fee_acc.clone(),
    //         self.system_program.to_account_info(),
    //      ],
    //     ).map_err(Into::into)
    // }
}

pub fn handler(ctx: Context<Unstake>, skip_rewards: bool) -> Result<()> {
    let farm = &ctx.accounts.farm;
    if ctx.accounts.farmer.state == FarmerState::Staked && farm.config.unstaking_fee_percent > 0 && farm.config.unstaking_fee_percent < 100 {
        //ctx.accounts.pay_treasury(farm.config.unstaking_fee_lamp)?
        farm.config.unstaking_fee_percent.try_div(100)?;
        token::transfer(
            ctx.accounts
            .pay_tokens_treasury_ctx()
            .with_signer(&[&ctx.accounts.farm.farm_seeds()]),
            farm.config.unstaking_fee_percent,
        )?;
    } 
    let farm = &mut ctx.accounts.farm;
    let farmer = &mut ctx.accounts.farmer;
    let now_ts = now_ts()?;
    if !skip_rewards {
        farm.update_rewards(now_ts, Some(farmer), false)?;
    }
    farm.end_staking(now_ts, farmer)?;
    if farmer.state == FarmerState::Unstaked {
        gem_bank::cpi::set_vault_lock(
            ctx.accounts.set_lock_vault_ctx()
                            .with_signer(&[&ctx.accounts.farm.farm_seeds()]), 
        false,)?;
    }
    
    //ctx.accounts.transfer_fee()?;
    Ok(())
}