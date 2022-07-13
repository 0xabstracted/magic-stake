use crate::state::*;
use anchor_lang::prelude::*;
use gem_common::TryAdd;

#[proc_macros::assert_size(160)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct FarmerLPPoints {
    pub lp_accrued: u64, //loyalty points
    pub lp_rate: FarmerLPRateReward,
    pub lp_level: u64,
    pub lp_levelup_rate: FarmerLPLevelupRate,
}

impl FarmerLPPoints {
    pub fn outstanding_lp_points(&self) -> Result<u64> {
        Ok(self.lp_accrued)
    }
    pub fn claim_lp_points(&mut self) -> Result<u64> {
        let outstanding = self.outstanding_lp_points()?;
        let to_claim = outstanding;
        Ok(to_claim)
    }
    pub fn update_lp_points(&mut self, now_ts: u64, newly_accured_lp: u64) -> Result<()> {
        self.lp_accrued.try_add_assign(newly_accured_lp)?;
        self.lp_rate.lp_last_updated_ts = self.lp_rate.lp_upper_bound(now_ts)?;
        Ok(())
    }
}
