use anchor_lang::prelude::*;

#[proc_macros::assert_size(8)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct FarmerLPLevelupRate {
    pub lp_tokens_levelup_threshold: u64,
}
