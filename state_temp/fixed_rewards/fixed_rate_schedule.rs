use super::HeldTenure;
use crate::state::tier_config::*;
use anchor_lang::prelude::*;
use gem_common::{TryAdd, TryDiv, TryMul, TrySub};

#[proc_macros::assert_size(88)]
#[repr(C)]
#[derive(Debug, Copy, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct FixedRateSchedule {
    pub base_rate: u64,
    pub tier1: Option<TierConfig>, //16 + 8 overhead
    pub tier2: Option<TierConfig>,
    pub tier3: Option<TierConfig>,
    /// needed to slowout the payout schedule (else min would be 1 token/rarity_point/sec or 86k/rairty_point/day)
    /// Only used in fixed rate, in variable overall duration serves as sufficient speed regulator
    pub denominator: u64,
}

/// custom impl of Defualt  becauase denominator cannot be 0 by default
impl Default for FixedRateSchedule {
    fn default() -> Self {
        Self {
            base_rate: 0,
            tier1: None,
            tier2: None,
            tier3: None,
            denominator: 1,
        }
    }
}

impl FixedRateSchedule {
    pub fn verify_schedule_invariants(&self) {
        if let Some(t3) = self.tier3 {
            //later tiers require earilier tiers to be present
            assert!(self.tier2.is_some() && self.tier1.is_some());
            let t2_tenure = self.tier2.unwrap().required_tenure;
            assert!(t3.required_tenure >= t2_tenure);
            let t1_tenure = self.tier1.unwrap().required_tenure;
            assert!(t2_tenure >= t1_tenure);
        };

        if let Some(t2) = self.tier2 {
            assert!(self.tier1.is_some());
            let t1_tenure = self.tier1.unwrap().required_tenure;
            assert!(t2.required_tenure >= t1_tenure)
        };
        assert_ne!(self.denominator, 0);
    }

    pub fn extract_tenure_and_rate(&self, tier: &str) -> Option<(u64, u64)> {
        match tier {
            "t1" => {
                if let Some(t) = self.tier1 {
                    Some((t.required_tenure, t.reward_rate))
                } else {
                    None
                }
            }
            "t2" => {
                if let Some(t) = self.tier2 {
                    Some((t.required_tenure, t.reward_rate))
                } else {
                    None
                }
            }
            "t3" => {
                if let Some(t) = self.tier1 {
                    Some((t.required_tenure, t.reward_rate))
                } else {
                    None
                }
            }
            _ => panic!("undefined tier"),
        }
    }

    pub fn get_base_reward(&self, start: u64, end: u64) -> Result<u64> {
        let duration = end.try_sub(start)?;
        self.base_rate.try_mul(duration)
    }

    fn extract_held_tenure(
        &self,
        tier: &str,
        start_from: u64,
        end_at: u64,
        max_end: &mut u64,
    ) -> Option<HeldTenure> {
        match self.extract_tenure_and_rate(tier) {
            // required_tenure acts as lower bound
            Some((begin, rate)) => {
                let ht = HeldTenure::new(rate, start_from, end_at, begin, *max_end);
                *max_end = begin;
                ht
            }
            _ => None,
        }
    }

    fn reward_per_rarity_point(&self, start_from: u64, end_at: u64) -> Result<u64> {
        let mut cap = u64::MAX;
        let t3 = self.extract_held_tenure("t3", start_from, end_at, &mut cap);
        let t2 = self.extract_held_tenure("t2", start_from, end_at, &mut cap);
        let t1 = self.extract_held_tenure("t1", start_from, end_at, &mut cap);

        let mut iter = vec![t1, t2, t3]
            .into_iter()
            .flatten()
            .map(|t| t.get_reward());
        let init = match start_from < cap {
            false => iter.next().unwrap(),
            true => self.get_base_reward(start_from, std::cmp::min(cap, end_at)),
        };
        iter.fold(init, |last, this| last?.try_add(this?))
    }

    pub fn reward_amount(&self, start_from: u64, end_at: u64, rarity_points: u64) -> Result<u64> {
        let per_rarity_point = self.reward_per_rarity_point(start_from, end_at)?;
        rarity_points
            .try_mul(per_rarity_point)?
            .try_div(self.denominator)
    }
}
