use crate::state::Farm;
use crate::state::Farmer;
use crate::state::ProbableRateSchedule;
use crate::state::LPRateSchedule;
use anchor_lang::prelude::*;
use gem_bank::{self, cpi::accounts::InitVault, program::GemBank, state::Bank};
use gem_common::TryAdd;


#[derive(Accounts)]
pub struct InitProbableFarmer<'info> {
    // farm
    #[account(mut, has_one = bank)]
    pub farm: Box<Account<'info, Farm>>,

    // farmer
    #[account(init, seeds = [
            b"farmer".as_ref(),
            farm.key().as_ref(),
            identity.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Farmer>())]
    pub farmer: Box<Account<'info, Farmer>>,
    pub identity: Signer<'info>,

    // cpi
    #[account(mut)]
    pub bank: Box<Account<'info, Bank>>,
    // trying to deserialize here leads to errors (doesn't exist yet)
    /// CHECK:
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub gem_bank: Program<'info, GemBank>,

    // misc
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitProbableFarmer<'info> {
    fn init_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, InitVault<'info>> {
        CpiContext::new(
            self.gem_bank.to_account_info(),
            InitVault {
                bank: self.bank.to_account_info(),
                vault: self.vault.clone(),
                // creator = the identity of the farmer
                creator: self.identity.to_account_info(),
                payer: self.payer.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<InitProbableFarmer>) -> Result<()> {
    // record new farmer details
    let farmer = &mut ctx.accounts.farmer;
    farmer.farm = ctx.accounts.farm.key();
    farmer.identity = ctx.accounts.identity.key();
    farmer.vault = ctx.accounts.vault.key();
    farmer.reward_a.probable_rate.promised_probable_schedule = ProbableRateSchedule::default();
    farmer.lp_points.lp_rate.lp_promised_schedule = LPRateSchedule::default();

    // update farm
    let farm = &mut ctx.accounts.farm;
    farm.farmer_count.try_add_assign(1)?;
    msg!("farmer.reward_a {:?}", farmer.reward_a);
    msg!("farmer.lp_points {:?}", farmer.lp_points);
    // do a cpi call to start a new vault
    let vault_owner = ctx.accounts.identity.key();
    let vault_name = String::from("farm_vault");
    gem_bank::cpi::init_vault(ctx.accounts.init_vault_ctx(), vault_owner, vault_name)?;
    msg!("new probable farmer initialized");
    Ok(())
}
