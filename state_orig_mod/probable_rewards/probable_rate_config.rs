use anchor_lang::prelude::*;

use crate::state::probable_rewards::probable_rate_schedule::*;

#[proc_macros::assert_size(136)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct ProbableRateConfig {
    pub probable_schedule: ProbableRateSchedule,
    /// total number of reward tokens that are being sent with the ix. Will be added on TOP of existing funding
    pub probable_amount: u64,
    /// duration the fund is being commited for
    /// if the funding is done for 100 sec
    pub probable_duration_sec: u64,
}
