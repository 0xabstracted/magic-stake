use crate::state::farmer::farmer_lp_points::*;
use crate::state::farmer::farmer_reward::*;
use crate::state::farmer::farmer_state::*;
use anchor_lang::prelude::*;
use gem_common::errors::ErrorCode;
use gem_common::TryAdd;

#[proc_macros::assert_size(712)]
#[repr(C)]
#[account]
#[derive(Debug)]
pub struct Farmer {
    pub farm: Pubkey, //32
    /// the identiy of the farmer - Publickey
    pub identity: Pubkey, //32
    /// vault storing all of farmer's gems
    pub vault: Pubkey, //32
    pub state: FarmerState, //4
    /// total number of gems when vault is locked
    pub gems_staked: u64, //8
    ///  sum of rairty points of all the NFTs staked. If not configured raritypoints are defualted to 1.
    pub rarity_points_staked: u64, //8
    /// this will be updated when they decide to unstake according to FarmConfig settings of Farm
    pub min_staking_end_ts: u64, //8
    /// this will be updated when they decide to unstake according to FarmConfig settings of Farm
    pub cooldown_end_ts: u64, //8
    pub reward_a: FarmerReward, //384
    //    pub reward_b: FarmerReward, //384
    pub lp_points: FarmerLPPoints, //160
    _reserved: [u8; 32],           //32
}

impl Farmer {
    pub fn begin_staking(
        &mut self,
        min_staking_period_sec: u64,
        now_ts: u64,
        gems_in_vault: u64, 
        rarity_points_in_vault: u64,
    ) -> Result<(u64, u64)> {
        self.state = FarmerState::Staked;
        let previous_gems_staked = self.gems_staked;
        let previous_rarity_points_staked = self.rarity_points_staked;
        self.gems_staked = gems_in_vault;
        self.rarity_points_staked = rarity_points_in_vault;
        self.min_staking_end_ts = now_ts.try_add(min_staking_period_sec)?;
        self.cooldown_end_ts = 0; // zero it out in case it was set before

        Ok((previous_gems_staked, previous_rarity_points_staked))
    }

    pub fn end_staking_begin_cooldown(
        &mut self,
        now_ts: u64,
        cooldown_period_sec: u64,
    ) -> Result<(u64, u64)> {
        if !self.can_end_cooldown(now_ts) {
            return Err(error!(ErrorCode::MinStakingNotPassed));
        }

        self.state = FarmerState::PendingCooldown;
        let gems_unstaked = self.gems_staked;
        let rarity_points_unstaked = self.rarity_points_staked;
        self.gems_staked = 0; //no rewards will accrue during the cooldown period
        self.rarity_points_staked = 0;
        self.cooldown_end_ts = now_ts.try_add(cooldown_period_sec)?;

        msg!(
           "{} gems are cooling down {}",
             gems_unstaked,
             self.identity,
        );
        Ok((gems_unstaked, rarity_points_unstaked))
    }

    pub fn end_cooldown(&mut self, now_ts: u64) -> Result<()> {
        if self.can_end_staking(now_ts) {
            return Err(error!(ErrorCode::CooldownNotPassed));
        }
        self.state = FarmerState::Unstaked;
        self.gems_staked = 0;
        self.rarity_points_staked = 0;
        self.cooldown_end_ts = 0;
        self.min_staking_end_ts = 0;
        // msg!(
        //    "gems now unstaked and available for withdrawal for {}",
        //      self.identity
        // );
        Ok(())
    }

    fn can_end_staking(&self, now_ts: u64) -> bool {
        now_ts >= self.min_staking_end_ts
    }

    fn can_end_cooldown(&self, now_ts: u64) -> bool {
        now_ts >= self.cooldown_end_ts
    }
}
