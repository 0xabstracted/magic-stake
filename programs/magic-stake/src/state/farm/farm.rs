use anchor_lang::prelude::*;
use gem_common::{errors::ErrorCode, *};

use crate::state::*;

pub const LATEST_FARM_VERSION: u16 = 0;

#[proc_macros::assert_size(928)]
#[repr(C)]
#[account]
#[derive(Debug)]
pub struct Farm {
    pub verison: u16, //2
    /// authorizes funders, whitelists mints/creators, sets farm config, can give away farm managing authority
    pub farm_manager: Pubkey, //32
    /// used for collecting any fees earned by the farm
    pub farm_treasury: Pubkey, //32
    /// signs off on treasury payouts and on any operations related to the bank (configured as bank manager)
    pub farm_authority: Pubkey, //32
    pub farm_authority_seed: Pubkey, //32
    pub farm_authority_seed_bump: [u8; 1], //1
    /// each farm controls a single bank. each farmer gets a vault in that bank
    pub bank: Pubkey, //32
    pub config: FarmConfig, //24
    pub farmer_count: u64, //8
    pub staked_farmer_count: u64, //8
    pub gems_staked: u64, //8
    pub rarity_points_staked: u64, //8
    pub authorized_funder_count: u64, //8
    pub reward_a: FarmReward, //512 440
    //    pub reward_b: FarmReward, //512
    pub lp_points: FarmLPPoints, //192
    pub max_counts: MaxCounts,   //12
    _reserved: [u8; 32],         //32
    _reserved2: [u8; 16],        //16
    _reserved3: [u8; 4],         //4
}

impl Farm {
    fn assert_valid_max_counts(&self) -> Result<()> {
        self.assert_not_too_many_farmers()?;
        self.assert_not_too_many_gems()?;
        self.assert_not_too_many_rairty_points()?;
        Ok(())
    }

    fn assert_not_too_many_farmers(&self) -> Result<()> {
        if self.max_counts.max_farmers > 0 {
            require!(
                self.staked_farmer_count.try_cast()? <= self.max_counts.max_farmers,
                ErrorCode::TooManyFarmersStaked
            )
        }
        Ok(())
    }

    fn assert_not_too_many_gems(&self) -> Result<()> {
        if self.max_counts.max_gems > 0 {
            require!(
                self.gems_staked.try_cast()? <= self.max_counts.max_gems,
                ErrorCode::TooManyGemsStaked
            )
        }
        Ok(())
    }

    fn assert_not_too_many_rairty_points(&self) -> Result<()> {
        if self.max_counts.max_rarity_points > 0 {
            require!(
                self.rarity_points_staked.try_cast()? <= self.max_counts.max_rarity_points,
                ErrorCode::TooManyRarityPointsStaked
            )
        }
        Ok(())
    }

    pub fn farm_seeds(&self) -> [&[u8]; 2] {
        [
            self.farm_authority_seed.as_ref(),
            &self.farm_authority_seed_bump,
        ]
    }

    pub fn match_reward_by_mint(&mut self, reward_mint: Pubkey) -> Result<&mut FarmReward> {
        let reward_a_mint = self.reward_a.reward_mint;
        //        let reward_b_mint = self.reward_b.reward_mint;

        match reward_mint {
            _ if reward_mint == reward_a_mint => Ok(&mut self.reward_a),
            //            _ if reward_mint == reward_b_mint => Ok(&mut self.reward_b),
            _ => Err(error!(ErrorCode::UnknownRewardMint)),
        }
    }

    pub fn lock_reward_by_mint(&mut self, reward_mint: Pubkey) -> Result<()> {
        let reward = self.match_reward_by_mint(reward_mint)?;
        reward.lock_reward()
    }

    pub fn fund_reward_by_mint(
        &mut self,
        now_ts: u64,
        reward_mint: Pubkey,
        //        variable_rate_config: Option<VariableRateConfig>,
        fixed_rate_config: Option<FixedRateConfig>,
        probable_rate_config: Option<ProbableRateConfig>,
    ) -> Result<()> {
        let reward = self.match_reward_by_mint(reward_mint)?;
        reward.fund_reward_by_type(
            now_ts,
            //            variable_rate_config,
            fixed_rate_config,
            probable_rate_config,
        )
    }

    pub fn cancel_reward_by_mint(&mut self, now_ts: u64, reward_mint: Pubkey) -> Result<u64> {
        let reward = self.match_reward_by_mint(reward_mint)?;
        reward.cancel_reward_by_type(now_ts)
    }

    pub fn update_rewards(
        &mut self,
        now_ts: u64,
        mut farmer: Option<&mut Account<Farmer>>,
        reenroll: bool,
    ) -> Result<()> {
        // reward_a
        let (farmer_rarity_points_staked, farmer_reward_a) = match farmer {
            Some(ref mut farmer) => (
                Some(farmer.rarity_points_staked),
                Some(&mut farmer.reward_a),
            ),
            None => (None, None),
        };

        self.reward_a.update_accrued_reward_by_type(
            now_ts,
            self.rarity_points_staked,
            farmer_rarity_points_staked,
            farmer_reward_a,
            reenroll,
        ) //?;

        // let farmer_reward_b = match farmer {
        //     Some(ref mut farmer) => Some(&mut farmer.reward_b),
        //     None => None,
        // };
        // self.reward_b.update_accrued_reward_by_type(
        //     now_ts,
        //     self.rarity_points_staked,
        //     farmer_points_staked,
        //     farmer_reward_b,
        //     reenroll,
        // )
    }
    
    pub fn start_lp_by_type(&mut self, now_ts: u64, lp_type: &str, lp_rate_config: Option<LPRateConfig>,) -> Result<()> {
        self.lp_points.start_lp_by_type(
            now_ts,
            lp_rate_config,
        )
    }
    pub fn cancel_lp_points_by_type(&self, now_ts: u64, lp_type: &str) -> Result<()> {
        let mut farm_lp_points = match lp_type {
            "RESPECT" => self.lp_points,
            _ => return Err(error!(ErrorCode::UnknownLPType)),
        };
        farm_lp_points.cancel_lp_points_by_type(now_ts)
    }

    pub fn update_lp_points(
        &mut self,
        now_ts: u64,
        mut farmer: Option<&mut Account<Farmer>>,
        reenroll: bool,
    ) -> Result<()> {
        let (farmer_rarity_points_staked, farmer_lp_points) = match farmer {
            Some(ref mut farmer) => (
                Some(farmer.rarity_points_staked),
                Some(&mut farmer.lp_points),
            ),
            None => (None, None),
        };
        self.lp_points.update_accrued_lp_by_type(
            now_ts,
            self.rarity_points_staked,
            farmer_rarity_points_staked,
            farmer_lp_points,
            reenroll,
        )
    }

    pub fn begin_staking(
        &mut self,
        now_ts: u64,
        gems_in_vault: u64,
        rarity_points_in_vault: u64,
        farmer: &mut Account<Farmer>,
    ) -> Result<()> {
        //update farmer
        farmer.begin_staking(
            self.config.min_staking_period_sec,
            now_ts,
            gems_in_vault,
            rarity_points_in_vault,
        )?;

        //update farm
        self.staked_farmer_count.try_add_assign(1)?;
        self.gems_staked.try_add_assign(gems_in_vault)?;
        self.rarity_points_staked
            .try_add_assign(rarity_points_in_vault)?;
        self.assert_valid_max_counts()?;

        if self.reward_a.reward_type == RewardType::Fixed {
            self.reward_a.fixed_rate_reward.enroll_farmer(
                now_ts,
                &mut self.reward_a.times,
                &mut self.reward_a.funds,
                farmer.rarity_points_staked,
                &mut farmer.reward_a,
                None,
            )?;
        }

        if self.reward_a.reward_type == RewardType::Probable {
            self.reward_a.probable_rate.enroll_probable_farmer(
                now_ts,
                &mut self.reward_a.times,
                &mut self.reward_a.funds,
                farmer.rarity_points_staked,
                &mut farmer.reward_a,
                None,
            )?;
        }
        if self.lp_points.lp_type == LPType::RESPECT {
            self.lp_points.lp_rate.enroll_lp_farmer(
                now_ts,
                &mut self.lp_points.times,
                farmer.rarity_points_staked,
                &mut farmer.lp_points,
                None,
            )?;
        }
        // if self.reward_b.reward_type == RewardType::Fixed {
        //     self.reward_b.fixed_rate_reward.enroll_farmer(
        //         now_ts,
        //         &mut self.reward_b.times,
        //         &mut self.reward_b.funds,
        //         farmer.rarity_points_staked,
        //         &mut farmer.reward_b,
        //         None,
        //     )?;
        // }

        Ok(())
    }

    pub fn end_staking(&mut self, now_ts: u64, farmer: &mut Account<Farmer>) -> Result<()> {
        match farmer.state {
            FarmerState::Unstaked => Ok(msg!("already unstaked!!")),
            FarmerState::Staked => {
                if self.reward_a.reward_type == RewardType::Fixed {
                    self.reward_a
                        .fixed_rate_reward
                        .graduate_farmer(farmer.rarity_points_staked, &mut farmer.reward_a)?;
                }
                if self.reward_a.reward_type == RewardType::Probable {
                    self.reward_a.probable_rate.graduate_probable_farmer(
                        farmer.rarity_points_staked,
                        &mut farmer.reward_a,
                    )?;
                }
                if self.lp_points.lp_type == LPType::RESPECT {
                    self.lp_points
                        .lp_rate
                        .graduate_lp_farmer(farmer.rarity_points_staked, &mut farmer.lp_points)?;
                }
                // if self.reward_b.reward_type == RewardType::Fixed {
                //     self.reward_b
                //         .fixed_rate_reward
                //         .graduate_farmer(farmer.rarity_points_staked, &mut farmer.reward_b)?;
                // }

                //update farmer
                let (gems_unstaked, rarity_points_unstaked) =
                    farmer.end_staking_begin_cooldown(now_ts, self.config.cooldown_period_sec)?;

                //update farm
                self.staked_farmer_count.try_sub_assign(1)?;
                self.rarity_points_staked
                    .try_sub_assign(rarity_points_unstaked)?;
                self.gems_staked.try_sub_assign(gems_unstaked)?;
                Ok(())
            }
            FarmerState::PendingCooldown => farmer.end_cooldown(now_ts),
        }
    }

    pub fn stake_extra_gems(
        &mut self,
        now_ts: u64,
        gems_in_vault: u64,
        rarity_points_in_vault: u64,
        extra_gems: u64,
        extra_rarity_points: u64,
        farmer: &mut Account<Farmer>,
    ) -> Result<()> {
        //update farmer
        let (_previous_gems, previous_rarity_points) = farmer.begin_staking(
            self.config.min_staking_period_sec,
            now_ts,
            gems_in_vault,
            rarity_points_in_vault,
        )?;

        //update farm
        self.gems_staked.try_add_assign(extra_gems)?;
        self.rarity_points_staked
            .try_add_assign(extra_rarity_points)?;
        self.assert_valid_max_counts()?;

        if self.reward_a.reward_type == RewardType::Fixed {
            // graduate farmer with previous rarity points count
            let original_begin_staking_ts = self
                .reward_a
                .fixed_rate_reward
                .graduate_farmer(previous_rarity_points, &mut farmer.reward_a)?;

            // re-enroll with NEW rarity points count
            self.reward_a.fixed_rate_reward.enroll_farmer(
                now_ts,
                &mut self.reward_a.times,
                &mut self.reward_a.funds,
                farmer.rarity_points_staked,
                &mut farmer.reward_a,
                Some(original_begin_staking_ts),
            )?;
        }

        if self.reward_a.reward_type == RewardType::Probable {
            // graduate farmer with previous rarity points count
            let original_begin_staking_ts = self
                .reward_a
                .probable_rate
                .graduate_probable_farmer(previous_rarity_points, &mut farmer.reward_a)?;

            // re-enroll with NEW rarity points count
            self.reward_a.probable_rate.enroll_probable_farmer(
                now_ts,
                &mut self.reward_a.times,
                &mut self.reward_a.funds,
                farmer.rarity_points_staked,
                &mut farmer.reward_a,
                Some(original_begin_staking_ts),
            )?;
        }
        if self.lp_points.lp_type == LPType::RESPECT {
            // graduate farmer with previous rarity points count
            let original_begin_staking_ts = self
                .lp_points
                .lp_rate
                .graduate_lp_farmer(previous_rarity_points, &mut farmer.lp_points)?;

            // re-enroll with NEW rarity points count
            self.lp_points.lp_rate.enroll_lp_farmer(
                now_ts,
                &mut self.lp_points.times,
                farmer.rarity_points_staked,
                &mut farmer.lp_points,
                Some(original_begin_staking_ts),
            )?;
        }
        // if self.reward_b.reward_type == RewardType::Fixed {
        //     let original_begin_staking_ts = self
        //         .reward_b
        //         .fixed_rate_reward
        //         .graduate_farmer(previous_rarity_points, &mut farmer.reward_b)?;

        //     self.reward_b.fixed_rate_reward.enroll_farmer(
        //         now_ts,
        //         &mut self.reward_b.times,
        //         &mut self.reward_b.funds,
        //         farmer.rarity_points_staked,
        //         &mut farmer.reward_b,
        //         Some(original_begin_staking_ts),
        //     )?;
        // }
        Ok(())
    }
}
