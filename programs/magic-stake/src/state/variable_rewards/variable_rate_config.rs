use anchor_lang::prelude::*;

#[proc_macros::assert_size(16)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct VariableRateConfig {
    /// total amount of reward
    pub amount: u64,
    /// reward is active for this duration
    pub duration_sec: u64,
}
