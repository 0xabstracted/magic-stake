// use std::str::FromStr;

use anchor_lang::prelude::*;
use gem_common::{errors::ErrorCode, TryAdd, TrySub, close_account};

pub const MAX_NFTS_ALLOWED: usize = 200;

// pub const default_staked_mint: Pubkey = Pubkey::default();

#[account(zero_copy)]
#[derive(Debug)]
pub struct FarmerStakedMints {
    pub bump : u8,
    pub farmer: Pubkey,
    pub index: u32,
    pub no_of_nfts_staked: u64,
    pub farmer_staked_mints: [Pubkey; MAX_NFTS_ALLOWED],
}

impl FarmerStakedMints {
    pub fn append_nft(&mut self, farmer_staked_mint: Pubkey) -> Result<()> {
        if self.no_of_nfts_staked >= MAX_NFTS_ALLOWED as u64 {
            return Err(error!(ErrorCode::NotEnoughSpaceForStakedMint));
        }
        let default_staked_mint: Pubkey = Pubkey::default();

        for i in 0..MAX_NFTS_ALLOWED{
            if self.farmer_staked_mints[i] == default_staked_mint && 
                self.farmer_staked_mints[i] != farmer_staked_mint &&
                farmer_staked_mint!= default_staked_mint
            {
                self.farmer_staked_mints[i] = farmer_staked_mint;
                self.no_of_nfts_staked.try_add(1)?;
            }
        }
        Ok(())
    }
    pub fn remove_nft(&mut self, farmer_staked_mint: Pubkey) -> Result<()> {
        if self.no_of_nfts_staked >= MAX_NFTS_ALLOWED as u64 {
            return Err(error!(ErrorCode::NotEnoughSpaceForStakedMint));
        }
        let default_staked_mint: Pubkey = Pubkey::default();
        for i in 0..MAX_NFTS_ALLOWED{
            if self.farmer_staked_mints[i] == farmer_staked_mint && 
                farmer_staked_mint!= default_staked_mint
            {
                self.farmer_staked_mints[i] = default_staked_mint;
                self.no_of_nfts_staked.try_sub(1)?;
                
            }
        }
        Ok(())
    }
}
