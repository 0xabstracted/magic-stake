use crate::number128::Number128;
use anchor_lang::prelude::*;

#[proc_macros::assert_size(32)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct FarmerVariableRateReward {
    /// used to keep track of how much of the variable reward has been updated for this farmer
    pub last_recorded_accrued_reward_per_rairty_point: Number128,
    _reserved: [u8; 16],
}
