use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gem_bank::{self, cpi::accounts::InitBank, program::GemBank};
use gem_common::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(bump_auth: u8, bump_treasury: u8)]
pub struct InitFixedFarm<'info> {
    // farm
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Farm>())]
    pub farm: Box<Account<'info, Farm>>,
    pub farm_manager: Signer<'info>,
    /// CHECK:
    #[account(mut, seeds = [farm.key().as_ref()], bump = bump_auth)]
    pub farm_authority: AccountInfo<'info>,

    // reward a
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
    #[account(init, seeds = [
            b"token_treasury".as_ref(),
            farm.key().as_ref(),
        ],
        bump,
        token::mint = reward_a_mint,
        token::authority = farm_authority,
        payer = payer)]
    pub farm_treasury_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub bank: Signer<'info>,
    pub gem_bank: Program<'info, GemBank>,

    // misc
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitFixedFarm<'info> {
    fn init_bank_ctx(&self) -> CpiContext<'_, '_, '_, 'info, InitBank<'info>> {
        CpiContext::new(
            self.gem_bank.to_account_info(),
            InitBank {
                bank: self.bank.to_account_info(),
                // using farm_authority, NOT farm_manager, coz on certain ixs (eg lock/unclock)
                // we need manager's sig, but we don't have farm manager's KP.
                // unfortunately this means we have to write a CPI ix for farm for each ix for bank
                bank_manager: self.farm_authority.clone(),
                payer: self.payer.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        )
    }
}

pub fn handler(
    ctx: Context<InitFixedFarm>,
    bump_auth: u8,
    lp_type: LPType,
    farm_config: FarmConfig,
    max_counts: Option<MaxCounts>,
) -> Result<()> {
    
    if farm_config.unstaking_fee_percent > 100 {
        return Err(error!(ErrorCode::InvalidUnstakingFee));
    }

    //record new farm details
    let farm = &mut ctx.accounts.farm;

    farm.version = LATEST_FARM_VERSION;
    farm.farm_manager = ctx.accounts.farm_manager.key();
    //    farm.farm_treasury = farm_treasury;
    farm.farm_treasury_token = ctx.accounts.farm_treasury_token.key();
    farm.farm_authority = ctx.accounts.farm_authority.key();
    farm.farm_authority_seed = farm.key();
    farm.farm_authority_bump_seed = [bump_auth];
    farm.bank = ctx.accounts.bank.key();
    farm.config = farm_config;

    farm.reward_a.reward_mint = ctx.accounts.reward_a_mint.key();
    farm.reward_a.reward_pot = ctx.accounts.reward_a_pot.key();
    farm.reward_a.reward_type = RewardType::Fixed;
    farm.reward_a.fixed_rate_reward.schedule = FixedRateSchedule::default();
    farm.lp_points.lp_type = lp_type;
    farm.lp_points.lp_rate.lp_schedule = LPRateSchedule::default();
    
    if let Some(max_counts) = max_counts {
        farm.max_counts = max_counts;
    }
    msg!("Init farm: config {:?}", farm.config);
    msg!("Init farm: reward_a {:?}", farm.reward_a);
    msg!("Init farm: lp_points {:?}", farm.lp_points);

    //do a cpi call to start a new bank
    gem_bank::cpi::init_bank(
        ctx.accounts
            .init_bank_ctx()
            .with_signer(&[&ctx.accounts.farm.farm_seeds()]),
    )?;
    //msg!("new farm initialized");
    Ok(())
}
