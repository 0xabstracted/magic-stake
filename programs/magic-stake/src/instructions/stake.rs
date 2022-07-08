use crate::state::Farm;
use crate::state::Farmer;
use anchor_lang::prelude::*;
use gem_bank::cpi::accounts::SetVaultLock;
use gem_bank::{
    program::GemBank,
    state::{Bank, Vault},
};
use gem_common::now_ts;
use gem_common::errors::ErrorCode;
// use anchor_lang::solana_program::{program::invoke, system_instruction};
// use crate::instructions::FEE_WALLET;
// use std::str::FromStr;
// const FEE_LAMPORTS: u64 = 2_000_000; // 0.002 SOL per stake/unstake


#[derive(Accounts)]
#[instruction(bump_auth: u8, bump_farmer: u8)]
pub struct Stake<'info> {
    #[account(mut, has_one = farm_authority, has_one = bank)]
    pub farm: Box<Account<'info, Farm>>,
    ///CHECK:
    #[account(mut, seeds = [farm.key().as_ref()], bump = bump_auth)]
    pub farm_authority: AccountInfo<'info>,
    #[account(mut, has_one = farm, has_one = identity, has_one = vault, seeds = [
        b"farmer".as_ref(),
        farm.key().as_ref(),
        identity.key().as_ref(),
    ], 
    bump = bump_farmer)]
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
    pub system_program: Program<'info, System>,
}

impl <'info> Stake <'info> {
    fn set_vault_lock_ctx(&self) -> CpiContext<'_, '_ , '_ ,'info, SetVaultLock<'info>> {
        CpiContext::new(
            self.gem_bank.to_account_info(),
            SetVaultLock { 
                bank: self.bank.to_account_info(), 
                bank_manager: self.farm_authority.clone(), 
                vault: self.vault.to_account_info(), 
            },
        )
    }

    // fn _transfer_fee(&self) -> Result<()> {
    //     invoke(
    //         &system_instruction::transfer(self.identity.key, self.fee_acc.key, FEE_LAMPORTS),
    //         &[
    //             self.identity.to_account_info(),
    //             self.fee_acc.clone(),
    //             self.system_program.to_account_info(),
    //         ],
    //     )
    //     .map_err(Into::into)
    // }
}

pub fn handler(ctx: Context<Stake>) -> Result<()> {
    if ctx.accounts.vault.gem_count == 0 {
        return Err(error!(ErrorCode::VaultIsEmpty));
    }
    gem_bank::cpi::set_vault_lock(
        ctx.accounts
        .set_vault_lock_ctx()
        .with_signer(&[&ctx.accounts.farm.farm_seeds()]),
        true,
    )?;
    let farm = &mut ctx.accounts.farm;
    let farmer = &mut ctx.accounts.farmer;
    let vault = &ctx.accounts.vault;
    let now_ts = now_ts()?;
    farm.update_rewards(now_ts, Some(farmer), true)?;
    farm.update_lp_points(now_ts, Some(farmer), true)?;
    farm.begin_staking(now_ts, vault.gem_count, vault.rarity_points, farmer)?;
 //   ctx.accounts.transfer_fee()?;
 //   msg!("{} gems staked by {}", farmer.gems_staked, farmer.key());
    Ok(())
}