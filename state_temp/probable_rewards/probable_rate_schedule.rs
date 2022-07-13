use crate::state::probable_rewards::probable_tier_config::*;
use crate::state::HeldTenure;
use anchor_lang::prelude::*;
use gem_common::{TryAdd, TryDiv, TryMul, TrySub};

#[proc_macros::assert_size(120)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct ProbableRateSchedule {
    pub prob1: ProbableTierConfig,
    pub prob2: Option<ProbableTierConfig>, //16 +8
    pub prob3: Option<ProbableTierConfig>,
    pub prob4: Option<ProbableTierConfig>,
    pub prob5: Option<ProbableTierConfig>,
    pub denominator: u64,
}

impl Default for ProbableRateSchedule {
    fn default() -> Self {
        Self {
            prob1: ProbableTierConfig {
                probable_reward_rate: 0,
                probability: 0,
            },
            prob2: None,
            prob3: None,
            prob4: None,
            prob5: None,
            denominator: 1,
        }
    }
}

impl ProbableRateSchedule {
    pub fn extract_prob_and_rate(&self, prob_tier: &str) -> Option<(u64, u64)> {
        match prob_tier {
            "prob1" => Some((self.prob1.probability, self.prob1.probable_reward_rate)),
            "prob2" => {
                if let Some(t) = self.prob2 {
                    Some((t.probability, t.probable_reward_rate))
                } else {
                    None
                }
            }
            "prob3" => {
                if let Some(t) = self.prob3 {
                    Some((t.probability, t.probable_reward_rate))
                } else {
                    None
                }
            }
            "prob4" => {
                if let Some(t) = self.prob4 {
                    Some((t.probability, t.probable_reward_rate))
                } else {
                    None
                }
            }
            "prob5" => {
                if let Some(t) = self.prob5 {
                    Some((t.probability, t.probable_reward_rate))
                } else {
                    None
                }
            }
            _ => panic!("undefined probability tier"),
        }
    }

    pub fn get_prob1_tier_reward(&self, start: u64, end: u64) -> Result<u64> {
        let duration = end.try_sub(start)?;
        self.prob1.probable_reward_rate.try_mul(duration)
    }

    fn probable_extract_held_tenure(
        &self,
        prob_tier: &str,
        start_from: u64,
        end_at: u64,
        max_end: &mut u64,
    ) -> Option<HeldTenure> {
        match self.extract_prob_and_rate(prob_tier) {
            // probability acts as lower bound
            Some((begin, rate)) => {
                let ht = HeldTenure::new(rate, start_from, end_at, begin, *max_end);
                *max_end = begin;
                ht
            }
            _ => None,
        }
    }

    fn probable_reward_per_rarity_point(&self, start_from: u64, end_at: u64) -> Result<u64> {
        let mut cap = u64::MAX;
        let t5 = self.probable_extract_held_tenure("prob5", start_from, end_at, &mut cap);
        let t4 = self.probable_extract_held_tenure("prob4", start_from, end_at, &mut cap);
        let t3 = self.probable_extract_held_tenure("prob3", start_from, end_at, &mut cap);
        let t2 = self.probable_extract_held_tenure("prob2", start_from, end_at, &mut cap);
        let t1 = self.probable_extract_held_tenure("prob1", start_from, end_at, &mut cap);

        let mut iter = vec![t1, t2, t3, t4, t5]
            .into_iter()
            .flatten()
            .map(|t| t.get_reward());
        let init = match start_from < cap {
            false => iter.next().unwrap(),
            true => self.get_prob1_tier_reward(start_from, std::cmp::min(cap, end_at)),
        };
        iter.fold(init, |last, this| last?.try_add(this?))
    }

    pub fn probable_reward_amount(
        &self,
        start_from: u64,
        end_at: u64,
        rarity_points: u64,
    ) -> Result<u64> {
        let per_rarity_point = self.probable_reward_per_rarity_point(start_from, end_at)?;
        rarity_points
            .try_mul(per_rarity_point)?
            .try_div(self.denominator)
    }
}
