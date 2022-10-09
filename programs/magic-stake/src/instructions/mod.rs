pub mod alpha_staking;
pub mod alpha_tokenswap;
pub mod farm_manager;
pub mod fixed_farm;
pub mod probable_farm;
pub mod stakor_user;

pub use alpha_staking::*;
pub use alpha_tokenswap::*;
pub use farm_manager::*;
pub use fixed_farm::*;
pub use probable_farm::*;
pub use stakor_user::*;
// have to duplicate or this won't show up in IDL

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq)]
pub struct RarityConfig {
    pub mint: Pubkey,
    pub rarity_points: u16,
}
