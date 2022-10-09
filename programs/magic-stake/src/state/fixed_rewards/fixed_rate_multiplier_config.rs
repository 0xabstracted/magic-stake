use anchor_lang::prelude::*;

#[proc_macros::assert_size(16)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct FixedRateMultiplierConfig {
    pub number_of_nfts: u64,
    pub extra_reward: u64,
}
