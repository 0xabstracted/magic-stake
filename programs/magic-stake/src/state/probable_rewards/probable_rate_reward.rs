use crate::state::{
    farmer::farmer_probable_rate_reward::*, probable_rewards::probable_rate_schedule::*,
    FarmerReward, FundsTracker, ProbableRateConfig, TimeTracker,
};
use anchor_lang::prelude::*;
use gem_common::errors::ErrorCode;
use gem_common::{TryAdd, TrySub};

#[proc_macros::assert_size(160)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct ProbableRateReward {
    // schedule is configured during funding
    pub probable_schedule: ProbableRateSchedule,
    pub reserved_amount: u64,
    _reserved: [u8; 32],
}
impl ProbableRateReward {
    pub fn fund_probable_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
        new_config: ProbableRateConfig,
    ) -> Result<()> {
        let ProbableRateConfig {
            probable_schedule,
            probable_amount,
            probable_duration_sec,
        } = new_config;

        times.duration_sec = probable_duration_sec;
        times.reward_end_ts = now_ts.try_add(probable_duration_sec)?;

        funds.total_funded.try_add_assign(probable_amount)?;
        self.probable_schedule = probable_schedule;
        //msg!("recorded new funding of {}", amount);
        Ok(())
    }

    pub fn cancel_probable_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
    ) -> Result<u64> {
        let refund_amount = funds.pending_amount()?.try_sub(self.reserved_amount)?;
        funds.total_funded.try_add_assign(refund_amount)?;
        times.end_reward(now_ts)?;
        // msg!("prepared a total refund amount of {}", refund_amount);
        Ok(refund_amount)
    }
    pub fn update_accrued_probable_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
        farmer_rarity_points_staked: u64,
        farmer_reward: &mut FarmerReward,
        reenroll: bool,
    ) -> Result<()> {
        let newly_accrued_reward = farmer_reward
            .probable_rate
            .newly_accrued_probable_reward(now_ts, farmer_rarity_points_staked)?;
        funds
            .total_accured_to_stakers
            .try_add_assign(newly_accrued_reward)?;
        self.reserved_amount.try_add_assign(newly_accrued_reward)?;
        farmer_reward.update_probable_reward(now_ts, newly_accrued_reward)?;
        if farmer_reward.probable_rate.is_staked()
            && farmer_reward.probable_rate.is_time_to_graduate(now_ts)?
        {
            let original_staking_start =
                self.graduate_probable_farmer(farmer_rarity_points_staked, farmer_reward)?;
            if reenroll {
                self.enroll_probable_farmer(
                    now_ts,
                    times,
                    funds,
                    farmer_rarity_points_staked,
                    farmer_reward,
                    Some(original_staking_start),
                )?;
            }
        }
        Ok(())
    }

    pub fn enroll_probable_farmer(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
        farmer_rarity_points_staked: u64,
        farmer_reward: &mut FarmerReward,
        original_staking_start: Option<u64>,
    ) -> Result<()> {
        let remaining_duration = times.remaining_duration(now_ts)?;
        //calc any bonus due to previous staking
        farmer_reward.probable_rate.begin_probable_staking_ts =
            original_staking_start.unwrap_or(now_ts);
        farmer_reward.probable_rate.begin_probable_schedule_ts = now_ts;
        let bonus_time = farmer_reward.probable_rate.loyal_staker_bonus_time()?;

        //calc how much we have to reserve for the farmer
        let reserve_amount = self.probable_schedule.probable_reward_amount(
            bonus_time,
            remaining_duration.try_add(bonus_time)?,
            farmer_rarity_points_staked,
        )?;
        if reserve_amount > funds.pending_amount()? {
            return Err(error!(ErrorCode::RewardUnderfunded));
        }

        //update farmer
        farmer_reward.probable_rate.probable_last_updated_ts = now_ts;
        farmer_reward.probable_rate.promised_probable_schedule = self.probable_schedule;
        farmer_reward.probable_rate.promised_probable_duration = remaining_duration;

        //update farm
        self.reserved_amount.try_add_assign(reserve_amount)?;
        Ok(())
    }

    pub fn graduate_probable_farmer(
        &mut self,
        farmer_rarity_points_staked: u64,
        farmer_reward: &mut FarmerReward,
    ) -> Result<u64> {
        let original_begin_staking_ts = farmer_reward.probable_rate.begin_probable_staking_ts;

        //reduce reserve amount
        let voided_reward = farmer_reward
            .probable_rate
            .voided_probable_reward(farmer_rarity_points_staked)?;
        self.reserved_amount.try_sub_assign(voided_reward)?;
        farmer_reward.probable_rate = FarmerProbableRateReward::default();
        // msg!("graduated farmer on {}", now_ts);
        Ok(original_begin_staking_ts)
    }
}
