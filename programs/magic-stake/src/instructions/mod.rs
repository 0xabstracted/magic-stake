pub mod alpha_tokenswap;
pub mod farm_manager;
pub mod stakor_user;
pub mod common;
pub mod vrf_actions;

pub use alpha_tokenswap::*;
pub use farm_manager::*;
pub use stakor_user::*;
pub use common::*;
pub use vrf_actions::*;


// have to duplicate or this won't show up in IDL

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq)]
pub struct RarityConfig {
    pub mint: Pubkey,
    pub rarity_points: u16,
}
