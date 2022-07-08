use crate::state::farm::LPType;
use crate::state::loyalty_rewards::LPRateSchedule;
use crate::state::{max_counts::MaxCounts, Farm, FarmConfig, RewardType};
use crate::state::{FixedRateSchedule, LATEST_FARM_VERSION};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gem_bank::{cpi::accounts::InitBank, program::GemBank};
use gem_common::errors::ErrorCode;
// use anchor_lang::solana_program::{program::invoke, system_instruction};
// use std::str::FromStr;

//pub const FEE_WALLET: &str = "2xhBxVVuXkdq2MRKerE9mr2s1szfHSedy21MVqf8gPoM"; //5th
//const FEE_LAMPORTS: u64 = 2_500_000_000; // 2.5 SOL per farm

#[derive(Accounts)]
#[instruction(bump_auth: u8, bump_treasury: u8)]
pub struct InitFarm<'info> {
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Farm>())]
    pub farm: Box<Account<'info, Farm>>,
    pub farm_manager: Signer<'info>,
    ///CHECK:
    #[account(mut, seeds = [farm.key().as_ref()], bump = bump_auth)]
    pub farm_authority: AccountInfo<'info>,
    #[account(init, seeds = [
            b"reward_pot".as_ref(),
            farm.key().as_ref(),
            reward_a_mint.key().as_ref(),
        ],
        bump,
        token::mint = reward_a_mint,
        token::authority = farm_authority,
        payer = payer)]
    pub reward_a_pot: Box<Account<'info, TokenAccount>>,
    pub reward_a_mint: Box<Account<'info, Mint>>,
    // #[account(init, seeds = [
    //         b"reward_pot".as_ref(),
    //         farm.key().as_ref(),
    //         reward_b_mint.key().as_ref(),
    //     ],
    //     bump,
    //     token::mint = reward_b_mint,
    //     token::authority = farm_authority,
    //     payer = payer)]
    // pub reward_b_pot: Box<Account<'info, TokenAccount>>,
    // pub reward_b_mint: Box<Account<'info, Mint>>,
    #[account(init, seeds = [
            b"token_treasury".as_ref(),
            farm.key().as_ref(),
            reward_a_mint.key().as_ref(),
        ],
        bump,
        token::mint = reward_a_mint,
        token::authority = farm_authority,
        payer = payer)]
    pub farm_treasury_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub bank: Signer<'info>,
    pub gem_bank: Program<'info, GemBank>,
    #[account(mut)]
    pub payer: Signer<'info>,
    // ///CHECK:
    // #[account(mut, address = Pubkey::from_str(FEE_WALLET).unwrap())]
    // pub fee_acc: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitFarm<'info> {
    fn init_bank_ctx(&self) -> CpiContext<'_, '_, '_, 'info, InitBank<'info>> {
        CpiContext::new(
            self.gem_bank.to_account_info(),
            InitBank {
                bank: self.bank.to_account_info(),
                bank_manager: self.farm_authority.clone(),
                payer: self.payer.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        )
    }

    // fn transfer_fee(&self) -> Result<()> {
    //     invoke(
    //         &system_instruction::transfer(self.payer.key, self.fee_acc.key, FEE_LAMPORTS),
    //         &[
    //             self.payer.to_account_info(),
    //             self.fee_acc.clone(),
    //             self.system_program.to_account_info(),
    //         ],
    //     )
    //     .map_err(Into::into)
    // }
}

pub fn handler(
    ctx: Context<InitFarm>,
    bump_auth: u8,
    reward_type_a: RewardType,
    lp_type: LPType,
    //    reward_type_b: RewardType,
    farm_config: FarmConfig,
    max_counts: Option<MaxCounts>,
    //    farm_treasury: Pubkey,
) -> Result<()> {
    // let (pk, _bump) = Pubkey::find_program_address(
    //     &[b"treasury".as_ref(), ctx.accounts.farm.key().as_ref()],
    //     ctx.program_id,
    // );
    // if farm_treasury.key() != pk {
    //     return Err(error!(ErrorCode::InvalidParameter));
    // }
    // if farm_config.unstaking_fee_tokens < 0 || farm_config.unstaking_fee_tokens > 1000000 {
    //     return Err(error!(ErrorCode::InvalidUnstakingFee));
    // }

    if farm_config.unstaking_fee_percent > 100 {
        return Err(error!(ErrorCode::InvalidUnstakingFee));
    }
    let farm = &mut ctx.accounts.farm;
    farm.verison = LATEST_FARM_VERSION;
    farm.farm_manager = ctx.accounts.farm_manager.key();
    //    farm.farm_treasury = farm_treasury;
    farm.farm_treasury = ctx.accounts.farm_treasury_token.key();
    farm.farm_authority = ctx.accounts.farm_authority.key();
    farm.farm_authority_seed = farm.key();
    farm.farm_authority_seed_bump = [bump_auth];
    farm.bank = ctx.accounts.bank.key();
    farm.config = farm_config;

    farm.reward_a.reward_mint = ctx.accounts.reward_a_mint.key();
    farm.reward_a.reward_pot = ctx.accounts.reward_a_pot.key();
    farm.reward_a.reward_type = reward_type_a;
    farm.reward_a.fixed_rate.schedule = FixedRateSchedule::default();
    farm.lp_points.lp_type = lp_type;
    farm.lp_points.lp_rate.lp_schedule = LPRateSchedule::default();

    // farm.reward_b.reward_mint = ctx.accounts.reward_b_mint.key();
    // farm.reward_b.reward_pot = ctx.accounts.reward_b_pot.key();
    // farm.reward_b.reward_type = reward_type_b;
    // farm.reward_b.fixed_rate.schedule = FixedRateSchedule::default();

    if let Some(max_counts) = max_counts {
        farm.max_counts = max_counts;
    }
    gem_bank::cpi::init_bank(
        ctx.accounts
            .init_bank_ctx()
            .with_signer(&[&ctx.accounts.farm.farm_seeds()]),
    )?;
    // ctx.accounts.transfer_fee()?;
    msg!("new farm initialized");
    Ok(())
}
