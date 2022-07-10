use super::lp_type::*;
use super::time_tracker::*;
use crate::state::farmer::FarmerLPPoints;
use crate::state::loyalty_rewards::lp_rate_reward::*;
use crate::state::lp_rate_config;
use anchor_lang::prelude::*;
use gem_common::errors::ErrorCode;
use lp_rate_config::LPRateConfig;

#[proc_macros::assert_size(192)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct FarmLPPoints {
    pub lp_type: LPType,       //4
    pub lp_rate: LPRateReward, //128
    pub times: TimeTracker,    //24
    _reserved: [u8; 32],       //32
}

impl FarmLPPoints {
    pub fn lock_lp_points(&mut self) -> Result<()> {
        self.times.lock_end_ts = self.times.reward_end_ts;
        Ok(())
    }

    pub fn is_locked(self, now_ts: u64) -> bool {
        now_ts < self.times.lock_end_ts
    }
    pub fn start_lp_by_type(&mut self, now_ts: u64, lp_rate_config: Option<LPRateConfig>) -> Result<()> {
        if self.is_locked(now_ts) {
            return Err(error!(ErrorCode::RewardLocked));
        }
        match self.lp_type {
        LPType::RESPECT => self.lp_rate.start_lp(
            now_ts,
            &mut self.times,
            lp_rate_config.unwrap(),
        ),
    }
    }
    pub fn cancel_lp_points_by_type(&mut self, now_ts: u64) -> Result<()> {
        if self.is_locked(now_ts) {
            return Err(error!(ErrorCode::LPLocked));
        }
        match self.lp_type {
            LPType::RESPECT => self.lp_rate.cancel_lp_points(now_ts, &mut self.times),
        }
    }

    pub fn update_accrued_lp_by_type(
        &mut self,
        now_ts: u64,
        _farm_rarity_points_staked: u64,
        farmer_rarity_points_staked: Option<u64>,
        farmer_lp: Option<&mut FarmerLPPoints>,
        reenroll: bool,
    ) -> Result<()> {
        match self.lp_type {
            LPType::RESPECT => {
                if farmer_lp.is_none() {
                    return Ok(());
                }
                self.lp_rate.update_accrued_lp_points(
                    now_ts,
                    &mut self.times,
                    farmer_rarity_points_staked.unwrap(),
                    farmer_lp.unwrap(),
                    reenroll,
                )
            }
        }
    }
}
