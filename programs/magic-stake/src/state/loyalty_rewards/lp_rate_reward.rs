use crate::state::LPRateConfig;
use crate::state::farmer::FarmerLPRateReward;
use crate::state::{loyalty_rewards::lp_rate_schedule::*, FarmerLPPoints, TimeTracker};
use anchor_lang::prelude::*;
use gem_common::{TryAdd, TrySub};


#[proc_macros::assert_size(128)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct LPRateReward {
    /// configured during funding
    pub lp_schedule: LPRateSchedule,
    pub lp_value: u64,
    _reserved: [u8; 32],
}

impl LPRateReward {
    pub fn start_lp(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        lp_rate_config: LPRateConfig,
    ) -> Result<()> {
        let LPRateConfig {
            lp_schedule,
            lp_duration_sec,
        } = lp_rate_config;
        lp_schedule.verify_schedule_invariants();

        times.duration_sec = lp_duration_sec;
        times.reward_end_ts = now_ts.try_add(lp_duration_sec)?;

        self.lp_schedule = lp_schedule;
        msg!("recorded new lp of for {} sec, schedule: {:?}", lp_duration_sec, lp_schedule);
        Ok(())
    }

    pub fn cancel_lp_points(&mut self, now_ts: u64, times: &mut TimeTracker) -> Result<()> {
        times.end_reward(now_ts)
    }
    pub fn update_accrued_lp_points(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        farmer_rarity_points_staked: u64,
        farmer_lp: &mut FarmerLPPoints,
        reenroll: bool,
    ) -> Result<()> {
        let newly_accrued_lp = farmer_lp
            .lp_rate
            .newly_accrued_lp(now_ts, farmer_rarity_points_staked)?;
        farmer_lp.update_lp_points(now_ts, newly_accrued_lp)?;
        if farmer_lp.lp_rate.is_staked() && farmer_lp.lp_rate.is_time_to_graduate(now_ts)? {
            let original_staking_start =
                self.graduate_lp_farmer(farmer_rarity_points_staked, farmer_lp)?;
            if reenroll {
                self.enroll_lp_farmer(
                    now_ts,
                    times,
                    farmer_rarity_points_staked,
                    farmer_lp,
                    Some(original_staking_start),
                )?;
            }
        }
        Ok(())
    }

    pub fn enroll_lp_farmer(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        farmer_rarity_points_staked: u64,
        farmer_lp: &mut FarmerLPPoints,
        original_staking_start: Option<u64>,
    ) -> Result<()> {
        let remaining_duration = times.remaining_duration(now_ts)?;
        //calc any bonus due to previous staking
        farmer_lp.lp_rate.lp_begin_staking_ts = original_staking_start.unwrap_or(now_ts);
        farmer_lp.lp_rate.lp_begin_schedule_ts = now_ts;
        let bonus_time = farmer_lp.lp_rate.loyal_staker_bonus_time()?;

        //calc how much we have to reserve for the farmer
        let reserve_amount = self.lp_schedule.lp_reward_amount(
            bonus_time,
            remaining_duration.try_add(bonus_time)?,
            farmer_rarity_points_staked,
        )?;
        //update farmer
        farmer_lp.lp_rate.lp_last_updated_ts = now_ts;
        farmer_lp.lp_rate.lp_promised_schedule = self.lp_schedule;
        farmer_lp.lp_rate.lp_promised_duration = remaining_duration;

        //update farm
        self.lp_value.try_add_assign(reserve_amount)?;
        Ok(())
    }

    pub fn graduate_lp_farmer(
        &mut self,
        farmer_rarity_points_staked: u64,
        farmer_lp: &mut FarmerLPPoints,
    ) -> Result<u64> {
        let original_begin_staking_ts = farmer_lp.lp_rate.lp_begin_staking_ts;

        //reduce reserve amount
        let voided_reward = farmer_lp.lp_rate.voided_lp(farmer_rarity_points_staked)?;
        self.lp_value.try_sub_assign(voided_reward)?;
        farmer_lp.lp_rate = FarmerLPRateReward::default();
        // msg!("graduated farmer on {}", now_ts);
        Ok(original_begin_staking_ts)
    }
}
