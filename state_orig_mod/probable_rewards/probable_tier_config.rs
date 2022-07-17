use anchor_lang::prelude::*;
#[proc_macros::assert_size(16)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct ProbableTierConfig {
    pub probable_reward_rate: u64,
    pub probability: u64,
}
