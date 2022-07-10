use crate::state::loyalty_rewards::lp_rate_schedule::*;
use anchor_lang::prelude::*;

#[proc_macros::assert_size(96)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct LPRateConfig {
    pub lp_schedule: LPRateSchedule,
    /// duration the fund is being commited for
    /// If funding is done for 100 sec and a farmer stakes 3 secs after the commitment, the farmer is gaurenteed
    /// 97 sec of reward tokens
    /// Every farmer enrolled to the farm will be reserved an amount of reward tokens for the schedule
    /// of the duration
    pub lp_duration_sec: u64,
}
