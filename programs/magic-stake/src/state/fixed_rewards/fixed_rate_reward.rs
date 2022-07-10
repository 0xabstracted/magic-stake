use anchor_lang::prelude::*;
use gem_common::{errors::ErrorCode, *};

use crate::state::farm::funds_tracker::*;
use crate::state::farm::time_tracker::*;
use crate::state::fixed_rewards::fixed_rate_config::*;
use crate::state::fixed_rewards::fixed_rate_schedule::*;
use crate::state::FarmerFixedRateReward;
use crate::state::FarmerReward;

#[proc_macros::assert_size(128)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct FixedRateReward {
    // configured during funding
    pub schedule: FixedRateSchedule,
    // amount that has been promised to existing stakers and hence can't be withdrawn.
    pub reserved_amount: u64,
    // reserved for future updates
    _reserved: [u8; 32],
}

impl FixedRateReward {
    pub fn fund_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
        new_config: FixedRateConfig,
    ) -> Result<()> {
        let FixedRateConfig {
            schedule,
            amount,
            duration_sec,
        } = new_config;
        schedule.verify_schedule_invariants();

        times.duration_sec = duration_sec;
        times.reward_end_ts = now_ts.try_add(duration_sec)?;

        funds.total_funded.try_add_assign(amount)?;
        self.schedule = schedule;
        msg!("recorded new funding of {} for {} sec, schedule: {:?}", amount, duration_sec, schedule);
        Ok(())
    }

    pub fn cancel_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
    ) -> Result<u64> {
        let refund_amount = funds.pending_amount()?.try_sub(self.reserved_amount)?;
        funds.total_refunded.try_add_assign(refund_amount)?;
        times.end_reward(now_ts)?;
        msg!("prepared a total refund amount of {} now_ts{}", refund_amount, now_ts);
        Ok(refund_amount)
    }

    pub fn update_accrued_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
        farmer_rarity_points_staked: u64,
        farmer_reward: &mut FarmerReward,
        reenroll: bool,
    ) -> Result<()> {
        let newly_accrued_reward = farmer_reward
            .fixed_reward
            .newly_accrued_reward(now_ts, farmer_rarity_points_staked)?;
        // update farm (move amount from reserved to accrued)
        funds
            .total_accured_to_stakers
            .try_add_assign(newly_accrued_reward)?;
        self.reserved_amount.try_add_assign(newly_accrued_reward)?;
        // update farmer
        farmer_reward.update_fixed_reward(now_ts, newly_accrued_reward)?;
        if farmer_reward.fixed_reward.is_staked()
            && farmer_reward.fixed_reward.is_time_to_graduate(now_ts)?
        {
            let original_staking_start =
                self.graduate_farmer(farmer_rarity_points_staked, farmer_reward)?;
            if reenroll {
                self.enroll_farmer(
                    now_ts,
                    times,
                    funds,
                    farmer_rarity_points_staked,
                    farmer_reward,
                    Some(original_staking_start),
                )?;
            }
        }
        msg!("newly_accrued_reward {} now_ts{} funds{:?} times{:?} farmer_rarity_points_staked{} farmer_reward{:?}", newly_accrued_reward, now_ts, funds, times, farmer_rarity_points_staked, farmer_reward);
        Ok(())
    }

    pub fn enroll_farmer(
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
        farmer_reward.fixed_reward.begin_staking_ts = original_staking_start.unwrap_or(now_ts);
        farmer_reward.fixed_reward.begin_schedule_ts = now_ts;
        let bonus_time = farmer_reward.fixed_reward.loyal_staker_bonus_time()?;

        //calc how much we have to reserve for the farmer
        let reserve_amount = self.schedule.reward_amount(
            bonus_time,
            remaining_duration.try_add(bonus_time)?,
            farmer_rarity_points_staked,
        )?;
        if reserve_amount > funds.pending_amount()? {
            return Err(error!(ErrorCode::RewardUnderfunded));
        }

        //update farmer
        farmer_reward.fixed_reward.last_updated_ts = now_ts;
        farmer_reward.fixed_reward.promised_schedule = self.schedule;
        farmer_reward.fixed_reward.promised_duration = remaining_duration;

        //update farm
        self.reserved_amount.try_add_assign(reserve_amount)?;
        Ok(())
    }

    pub fn graduate_farmer(
        &mut self,
        farmer_rarity_points_staked: u64,
        farmer_reward: &mut FarmerReward,
    ) -> Result<u64> {
        let original_begin_staking_ts = farmer_reward.fixed_reward.begin_staking_ts;

        //reduce reserve amount
        let voided_reward = farmer_reward
            .fixed_reward
            .voided_reward(farmer_rarity_points_staked)?;
        self.reserved_amount.try_sub_assign(voided_reward)?;
        farmer_reward.fixed_reward = FarmerFixedRateReward::default();
        // msg!("graduated farmer on {}", now_ts);
        Ok(original_begin_staking_ts)
    }
}
