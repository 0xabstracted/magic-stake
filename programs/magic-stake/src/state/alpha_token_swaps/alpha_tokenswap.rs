use anchor_lang::prelude::*;

#[proc_macros::assert_size(96)]
#[repr(C)]
#[account]
#[derive(Debug)]
pub struct AlphaTokenswap {
    pub alpha_creator: Pubkey,
    pub alpha_mint: Pubkey,
    pub alpha_pot: Pubkey,
}
