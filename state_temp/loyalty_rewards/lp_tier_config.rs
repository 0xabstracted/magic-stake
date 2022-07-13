use anchor_lang::prelude::*;

#[proc_macros::assert_size(16)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct LPTierConfig {
    /// LP per denominator per rarity_point / sec
    pub lp_tier_rate: u64,
    /// minimum amount of time that needs to pass for the above rate to come into effect
    pub lp_required_tenure: u64,
}
