use super::LPTierConfig;
use crate::state::*;
use anchor_lang::prelude::*;
use gem_common::{TryAdd, TryDiv, TryMul, TrySub};

#[proc_macros::assert_size(88)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct LPRateSchedule {
    pub lp_base_rate: u64,
    pub lp_tier1: Option<LPTierConfig>, //16 + 8 overhead
    pub lp_tier2: Option<LPTierConfig>,
    pub lp_tier3: Option<LPTierConfig>,
    /// needed to slowout the payout schedule (else min would be 1 LP/rarity_point/sec or 86kLP/rairty_point/day)
    pub lp_denominator: u64,
}

/// custom impl of Defualt  becauase denominator cannot be 0 by default
impl Default for LPRateSchedule {
    fn default() -> Self {
        Self {
            lp_base_rate: 0,
            lp_tier1: None,
            lp_tier2: None,
            lp_tier3: None,
            lp_denominator: 1,
        }
    }
}

impl LPRateSchedule {
    pub fn verify_schedule_invariants(&self) {
        if let Some(t3) = self.lp_tier3 {
            assert!(self.lp_tier2.is_some() && self.lp_tier1.is_some());
            let t2_tenure = self.lp_tier2.unwrap().lp_required_tenure;
            assert!(t3.lp_required_tenure >= t2_tenure);
            let t1_tenure = self.lp_tier1.unwrap().lp_required_tenure;
            assert!(t2_tenure >= t1_tenure);
        };
        if let Some(t2) = self.lp_tier2 {
            assert!(self.lp_tier1.is_some());
            let t1_tenure = self.lp_tier1.unwrap().lp_required_tenure;
            assert!(t2.lp_required_tenure >= t1_tenure);
        };
        assert_ne!(self.lp_denominator, 0);
    }

    pub fn extract_tenure_and_rate(&self, lp_tier: &str) -> Option<(u64, u64)> {
        match lp_tier {
            "lp_t1" => {
                if let Some(t) = self.lp_tier1 {
                    Some((t.lp_required_tenure, t.lp_tier_rate))
                } else {
                    None
                }
            }
            "lp_t2" => {
                if let Some(t) = self.lp_tier2 {
                    Some((t.lp_required_tenure, t.lp_tier_rate))
                } else {
                    None
                }
            }
            "lp_t3" => {
                if let Some(t) = self.lp_tier3 {
                    Some((t.lp_required_tenure, t.lp_tier_rate))
                } else {
                    None
                }
            }
            _ => panic!("undefined lp_tier"),
        }
    }

    pub fn get_base_lp_reward(&self, start: u64, end: u64) -> Result<u64> {
        let duration = end.try_sub(start)?;
        self.lp_base_rate.try_mul(duration)
    }

    fn extract_held_tenure(
        &self,
        lp_tier: &str,
        start_from: u64,
        end_at: u64,
        max_end: &mut u64,
    ) -> Option<HeldTenure> {
        match self.extract_tenure_and_rate(lp_tier) {
            Some((begin, rate)) => {
                let ht = HeldTenure::new(rate, start_from, end_at, begin, *max_end);
                *max_end = begin;
                ht
            }
            _ => None,
        }
    }

    pub fn lp_reward_per_rarity_point(&self, start_from: u64, end_at: u64) -> Result<u64> {
        let mut cap = u64::MAX;
        let t3 = self.extract_held_tenure("lp_t3", start_from, end_at, &mut cap);
        let t2 = self.extract_held_tenure("lp_t2", start_from, end_at, &mut cap);
        let t1 = self.extract_held_tenure("lp_t1", start_from, end_at, &mut cap);
        let mut iter = vec![t1, t2, t3]
            .into_iter()
            .flatten()
            .map(|t| t.get_reward());
        let init = match start_from < cap {
            false => iter.next().unwrap(),
            true => self.get_base_lp_reward(start_from, std::cmp::min(cap, end_at)),
        };
        iter.fold(init, |last, this| last?.try_add(this?))
    }

    pub fn lp_reward_amount(
        &self,
        start_from: u64,
        end_at: u64,
        rarity_points: u64,
    ) -> Result<u64> {
        let per_rarity_point = self.lp_reward_per_rarity_point(start_from, end_at)?;
        rarity_points
            .try_mul(per_rarity_point)?
            .try_div(self.lp_denominator)
    }
}
