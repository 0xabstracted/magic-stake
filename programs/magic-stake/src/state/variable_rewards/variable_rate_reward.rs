use anchor_lang::prelude::*;
use farmer_reward::FarmerReward;
use gem_common::{TryAdd, TryDiv, TryMul, TrySub};

use crate::{
    number128::*,
    state::{farmer_reward, FundsTracker, TimeTracker, VariableRateConfig},
};

#[proc_macros::assert_size(72)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct VariableRateReward {
    pub reward_rate: Number128,
    pub reward_last_updated_ts: u64,
    pub accrued_reward_per_rarity_point: Number128,
    _reserved: [u8; 32],
}

impl VariableRateReward {
    pub fn fund_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
        new_config: VariableRateConfig,
    ) -> Result<()> {
        let VariableRateConfig {
            amount,
            duration_sec,
        } = new_config;

        // if previous reward has been exhauseted
        if now_ts > times.reward_end_ts {
            self.reward_rate = Number128::from(amount).try_div(Number128::from(duration_sec))?;
        }
        //else previous reward is still active
        else {
            self.reward_rate = Number128::from(amount)
                .try_add(Number128::from(funds.pending_amount()?))?
                .try_div(Number128::from(duration_sec))?;
        }

        times.duration_sec = duration_sec;
        times.reward_end_ts = now_ts.try_add(times.duration_sec)?;
        funds.total_funded.try_add_assign(amount)?;

        self.reward_last_updated_ts = times.reward_upper_bound(now_ts);
        //msg!("recorded new funding of {}", amount);

        Ok(())
    }

    pub fn cancel_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
    ) -> Result<u64> {
        let refund_amount = funds.pending_amount()?;

        funds.total_refunded.try_add_assign(refund_amount)?;
        times.end_reward(now_ts)?;

        self.reward_rate = Number128::ZERO;
        self.reward_last_updated_ts = times.reward_upper_bound(now_ts);

        Ok(refund_amount)
    }

    pub fn update_accrued_reward(
        &mut self,
        now_ts: u64,
        times: &mut TimeTracker,
        funds: &mut FundsTracker,
        farm_rarity_points_staked: u64,
        farmer_rarity_points_staked: Option<u64>,
        farmer_reward: Option<&mut FarmerReward>,
    ) -> Result<()> {
        let reward_upper_bound = times.reward_upper_bound(now_ts);

        //calc and update reward per rarity point
        let newly_accured_reward_per_rarity_point = self
            .newly_accrued_reward_per_rarity_point(farm_rarity_points_staked, reward_upper_bound)?;
        self.accrued_reward_per_rarity_point
            .try_add_assign(newly_accured_reward_per_rarity_point)?;

        //update overall reward
        funds.total_accured_to_stakers.try_add_assign(
            newly_accured_reward_per_rarity_point
                .try_mul(Number128::from(farm_rarity_points_staked))?
                .as_u64_ceil(0)?,
        )?;

        //update farmer, if one was passed

        if let Some(farmer_reward) = farmer_reward {
            let newly_accrued_to_farmer = Number128::from(farmer_rarity_points_staked.unwrap())
                .try_mul(
                    self.accrued_reward_per_rarity_point.try_sub(
                        farmer_reward
                            .variable_reward
                            .last_recorded_accrued_reward_per_rairty_point,
                    )?,
                )?;
            farmer_reward.update_variable_reward(
                newly_accrued_to_farmer.as_u64(0)?,
                self.accrued_reward_per_rarity_point,
            )?;
        }
        self.reward_last_updated_ts = reward_upper_bound;

        // msg!("updated reward as of {}", self.reward_last_updated_ts);
        Ok(())
    }

    fn newly_accrued_reward_per_rarity_point(
        &self,
        farm_rarity_points_staked: u64,
        reward_upper_bound: u64,
    ) -> Result<Number128> {
        if farm_rarity_points_staked == 0 {
            msg!("No gems are staked at the farm, means no rewards accrue");
            return Ok(Number128::ZERO);
        }
        let time_since_last_calc = reward_upper_bound.try_sub(self.reward_last_updated_ts)?;
        Number128::from(time_since_last_calc)
            .try_mul(self.reward_rate)?
            .try_div(Number128::from(farm_rarity_points_staked))
    }
}
