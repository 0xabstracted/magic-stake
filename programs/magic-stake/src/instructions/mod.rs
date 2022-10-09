pub mod add_rarities_to_bank;
pub mod add_to_bank_whitelist;
pub mod authorize_funder;
pub mod cancel_reward;
pub mod claim;
pub mod deauthorize_funder;
pub mod flash_deposit;
pub mod fund_reward;
pub mod init_fixed_farm;
pub mod init_probable_farm;
pub mod init_fixed_farmer;
pub mod init_probable_farmer;
pub mod lock_reward;
pub mod refresh_farmer;
pub mod refresh_farmer_signed;
pub mod remove_from_bank_whitelist;
pub mod stake;
pub mod treasury_payout;
pub mod unstake;
pub mod update_farm;
pub mod alpha_tokenswap;
pub mod alpha_staking;

pub use add_rarities_to_bank::*;
pub use add_to_bank_whitelist::*;
pub use authorize_funder::*;
pub use cancel_reward::*;
pub use claim::*;
pub use deauthorize_funder::*;
pub use flash_deposit::*;
pub use fund_reward::*;
pub use init_fixed_farm::*;
pub use init_probable_farm::*;
pub use init_fixed_farmer::*;
pub use init_probable_farmer::*;
pub use lock_reward::*;
pub use refresh_farmer::*;
pub use refresh_farmer_signed::*;
pub use remove_from_bank_whitelist::*;
pub use stake::*;
pub use treasury_payout::*;
pub use unstake::*;
pub use update_farm::*;
pub use alpha_tokenswap::*;
pub use alpha_staking::*;
// have to duplicate or this won't show up in IDL

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq)]
pub struct RarityConfig {
    pub mint: Pubkey,
    pub rarity_points: u16,
}
