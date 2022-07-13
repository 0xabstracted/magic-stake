use crate::state::loyalty_rewards::lp_rate_schedule::*;
use anchor_lang::prelude::*;
use gem_common::{TryAdd, TrySub};

#[proc_macros::assert_size(136)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, AnchorDeserialize, AnchorSerialize)]
pub struct FarmerLPRateReward {
    /// this is the time the farmer has staked.
    /// can be WAY BACK in past, if we have rolled multiple times
    pub lp_begin_staking_ts: u64,
    /// this is the time the latest reward_schedule farmers subscribed to begins
    /// this + promised_duration = end_schedule_ts
    pub lp_begin_schedule_ts: u64,
    /// always set upper bound, not just now_ts
    pub lp_last_updated_ts: u64,
    pub lp_promised_schedule: LPRateSchedule,
    pub lp_promised_duration: u64,
    _reserved: [u8; 16],
}

impl FarmerLPRateReward {
    pub fn loyal_staker_bonus_time(&self) -> Result<u64> {
        self.lp_begin_schedule_ts.try_sub(self.lp_begin_staking_ts)
    }

    pub fn end_schedule_ts(&self) -> Result<u64> {
        self.lp_begin_schedule_ts.try_add(self.lp_promised_duration)
    }

    pub fn is_staked(&self) -> bool {
        // these get zeroed out when farmer graduates
        self.lp_begin_staking_ts > 0 && self.lp_begin_schedule_ts > 0
    }

    pub fn is_time_to_graduate(&self, now_ts: u64) -> Result<bool> {
        Ok(now_ts >= self.end_schedule_ts()?)
    }

    pub fn lp_upper_bound(&self, now_ts: u64) -> Result<u64> {
        Ok(std::cmp::min(now_ts, self.end_schedule_ts()?))
    }

    pub fn time_from_staking_to_update(&self) -> Result<u64> {
        self.lp_last_updated_ts.try_sub(self.lp_begin_staking_ts)
    }

    /// (!) intentionally uses begin_staking_ts for both start_from and end_at
    /// in doing so we increase both start_from and end_at by exactly loyal_staker_bonus_time
    pub fn voided_lp(&self, rarity_points: u64) -> Result<u64> {
        let start_from = self.time_from_staking_to_update()?;
        let end_at = self.end_schedule_ts()?.try_sub(self.lp_begin_staking_ts)?;

        self.lp_promised_schedule
            .lp_reward_amount(start_from, end_at, rarity_points)
    }

    /// (!) intentionally uses begin_staking_ts for both start_from and end_at
    /// in doing so we increase both start_from and end_at by exactly loyal_staker_bonus_time
    pub fn newly_accrued_lp(&self, now_ts: u64, rarity_points: u64) -> Result<u64> {
        let start_from = self.time_from_staking_to_update()?;
        let end_at = self
            .lp_upper_bound(now_ts)?
            .try_sub(self.lp_begin_staking_ts)?;

        self.lp_promised_schedule
            .lp_reward_amount(start_from, end_at, rarity_points)
    }
}
