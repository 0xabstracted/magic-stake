use anchor_lang::prelude::*;
pub mod instructions;
pub mod number128;
pub mod state;
pub mod vrf_actions;

use gem_bank::instructions::record_rarity_points::RarityConfig;
use instructions::*;
use state::*;
use vrf_actions::*;

declare_id!("ETiwBTiVf2wNApe5Qc1sGozV3nkUDKNRVuCWzJSFWWfV");

#[program]
pub mod magic_stake {
    use super::*;
    pub fn init_farm_alpha(
        ctx: Context<InitFarmAlpha>,
        bump_auth: u8,
        _bump_treasury_token: u8,
        //reward_type_b: RewardType,
        farm_config: FarmConfig,
        max_counts: Option<MaxCounts>,
        farm_treasury_token: Pubkey,
) -> Result<()> {
        msg!("init farm");
        instructions::init_farm_alpha::handler(
            ctx,
            bump_auth,
            farm_config,
            max_counts,
            farm_treasury_token
        )
    }
    pub fn init_fixed_farm(
        ctx: Context<InitFixedFarm>,
        bump_auth: u8,
        _bump_treasury_token: u8,
        lp_type: LPType,
        farm_config: FarmConfig,
        max_counts: Option<MaxCounts>,
    ) -> Result<()> {
        msg!("init fixed farm");
        instructions::init_fixed_farm::handler(
            ctx,
            bump_auth,
            lp_type,
            farm_config,
            max_counts,
        )
    }
    pub fn init_probable_farm(
        ctx: Context<InitProbableFarm>,
        bump_auth: u8,
        _bump_treasury_token: u8,
        lp_type: LPType,
        farm_config: FarmConfig,
        max_counts: Option<MaxCounts>,
    ) -> Result<()> {
        msg!("init farm");
        instructions::init_probable_farm::handler(
            ctx,
            bump_auth,
            lp_type,
            farm_config,
            max_counts,
        )
    }
    pub fn update_farm(
        ctx: Context<UpdateFarm>,
        config: Option<FarmConfig>,
        manager: Option<Pubkey>,
        max_counts: Option<MaxCounts>,
    ) -> Result<()> {
        instructions::update_farm::handler(ctx, config, manager, max_counts)
    }

    pub fn payout_from_treasury(
        ctx: Context<TreasuryPayout>,
        _bump_auth: u8,
        bump_treasury_token: u8,
        lamports: u64,
    ) -> Result<()> {
        msg!("payout");
        instructions::treasury_payout::handler(ctx, bump_treasury_token, lamports)
    }

    pub fn add_to_bank_whitelist(
        ctx: Context<AddToBankWhitelist>,
        _bump_auth: u8,
        whitelist_type: u8,
    ) -> Result<()> {
        msg!("add to bank whitelist");
        instructions::add_to_bank_whitelist::handler(ctx, whitelist_type)
    }

    pub fn remove_from_bank_whitelist(
        ctx: Context<RemoveFromBankWhitelist>,
        _bump_auth: u8,
        bump_wl: u8,
    ) -> Result<()> {
        msg!("remove from bank whitelist");
        instructions::remove_from_bank_whitelist::handler(ctx, bump_wl)
    }

    // --------------------------------------- farmer ops

    // pub fn init_farmer(ctx: Context<InitFarmer>) -> Result<()> {
    //     msg!("init farmer");
    //     instructions::init_farmer::handler(ctx)
    // }
    pub fn init_farmer_alpha(ctx: Context<InitFarmerAlpha>) -> Result<()> {
        msg!("init farmer alpha");
        instructions::init_farmer_alpha::handler(ctx)
    }
    pub fn init_fixed_farmer(ctx: Context<InitFixedFarmer>) -> Result<()> {
        msg!("init farmer fixed");
        instructions::init_fixed_farmer::handler(ctx)
    }
    pub fn init_probable_farmer(ctx: Context<InitProbableFarmer>) -> Result<()> {
        msg!("init farmer probable");
        instructions::init_probable_farmer::handler(ctx)
    }
    
    pub fn stake(ctx: Context<Stake>, _bump_auth: u8, _bump_farmer: u8) -> Result<()> {
        msg!("stake");
        instructions::stake::handler(ctx)
    }
    pub fn stake_alpha(ctx: Context<StakeAlpha>, _bump_auth: u8, _bump_farmer: u8) -> Result<()> {
        msg!("stake");
        instructions::stake_alpha::handler(ctx)
    }

    pub fn unstake(
        ctx: Context<Unstake>,
        _bump_auth: u8,
        _bump_treasury_token: u8,
        _bump_farmer: u8,
        skip_rewards: bool,
    ) -> Result<()> {
        msg!("unstake");
        instructions::unstake::handler(ctx, skip_rewards)
    }

    pub fn unstake_alpha(
        ctx: Context<UnstakeAlpha>,
        _bump_auth: u8,
        _bump_treasury_token: u8,
        _bump_farmer: u8,
        skip_rewards: bool,
    ) -> Result<()> {
        msg!("unstake alpha");
        instructions::unstake_alpha::handler(ctx, skip_rewards)
    }

    pub fn claim(
        ctx: Context<Claim>,
        _bump_auth: u8,
        _bump_farmer: u8,
        _bump_pot_a: u8,
        _bump_pot_b: u8,
    ) -> Result<()> {
        msg!("claim");
        instructions::claim::handler(ctx)
    }

    pub fn claim_alpha(
        ctx: Context<ClaimAlpha>,
        _bump_auth: u8,
        _bump_farmer: u8,
        _bump_pot_a: u8,
        _bump_pot_b: u8,
    ) -> Result<()> {
        msg!("claim alpha");
        instructions::claim_alpha::handler(ctx)
    }


    pub fn flash_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, FlashDeposit<'info>>,
        _bump_farmer: u8,
        bump_vault_auth: u8,
        bump_rarity: u8,
        amount: u64,
    ) -> Result<()> {
        // msg!("flash deposit"); //have to remove all msgs! or run out of compute budget for this ix
        instructions::flash_deposit::handler(ctx, bump_vault_auth, bump_rarity, amount)
    }

    pub fn flash_deposit_alpha<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, FlashDepositAlpha<'info>>,
        _bump_farmer: u8,
        bump_vault_auth: u8,
        bump_rarity: u8,
        amount: u64,
    ) -> Result<()> {
        // msg!("flash deposit"); //have to remove all msgs! or run out of compute budget for this ix
        instructions::flash_deposit_alpha::handler(ctx, bump_vault_auth, bump_rarity, amount)
    }

    pub fn refresh_farmer(ctx: Context<RefreshFarmer>, _bump: u8) -> Result<()> {
        msg!("refresh farmer");
        instructions::refresh_farmer::handler(ctx)
    }

    /// this one needs to be called by the farmer themselves
    /// it's useful if for some reason they can't re-enroll in another fixed reward cycle (eg reward exhausted)
    /// but they want to be able to refresh themselves and claim their earned rewards up to this point
    pub fn refresh_farmer_signed(
        ctx: Context<RefreshFarmerSigned>,
        _bump: u8,
        reenroll: bool,
    ) -> Result<()> {
        msg!("refresh farmer signed");
        instructions::refresh_farmer_signed::handler(ctx, reenroll)
    }

    pub fn refresh_farmer_alpha(ctx: Context<RefreshFarmerAlpha>, _bump: u8) -> Result<()> {
        msg!("refresh farmer alpha");
        instructions::refresh_farmer_alpha::handler(ctx)
    }

    /// this one needs to be called by the farmer themselves
    /// it's useful if for some reason they can't re-enroll in another fixed reward cycle (eg reward exhausted)
    /// but they want to be able to refresh themselves and claim their earned rewards up to this point
    pub fn refresh_farmer_signed_alpha(
        ctx: Context<RefreshFarmerSignedAlpha>,
        _bump: u8,
        reenroll: bool,
    ) -> Result<()> {
        msg!("refresh farmer signed alpha");
        instructions::refresh_farmer_signed_alpha::handler(ctx, reenroll)
    }


    // --------------------------------------- funder ops

    pub fn authorize_funder(ctx: Context<AuthorizeFunder>) -> Result<()> {
        msg!("authorize funder");
        instructions::authorize_funder::handler(ctx)
    }

    pub fn deauthorize_funder(ctx: Context<DeauthorizeFunder>, _bump: u8) -> Result<()> {
        msg!("feauthorize funder");
        instructions::deauthorize_funder::handler(ctx)
    }

    // --------------------------------------- reward ops

    pub fn fund_reward(
        ctx: Context<FundReward>,
        _bump_proof: u8,
        _bump_pot: u8,
        //  variable_rate_config: Option<VariableRateConfig>,
        fixed_rate_config: Option<FixedRateConfig>,
        probable_rate_config: Option<ProbableRateConfig>,
        lp_rate_config: Option<LPRateConfig>,
    ) -> Result<()> {
        msg!("fund reward");
        instructions::fund_reward::handler(
            ctx,
            /*variable_rate_config,*/ fixed_rate_config,
            probable_rate_config,
            lp_rate_config,
        )
    }

    pub fn cancel_reward(ctx: Context<CancelReward>, _bump_auth: u8, _bump_pot: u8) -> Result<()> {
        msg!("cancel reward");
        instructions::cancel_reward::handler(ctx)
    }

    pub fn fund_reward_alpha(
        ctx: Context<FundRewardAlpha>,
        _bump_proof: u8,
        _bump_pot: u8,
        fixed_rate_config: Option<FixedRateConfig>,
        fixed_rate_multiplier_config: Option<FixedRateMultiplierConfig>
) -> Result<()> {
        msg!("fund reward alpha");
        instructions::fund_reward_alpha::handler(
            ctx,
            fixed_rate_config,
            fixed_rate_multiplier_config
        )
    }

    pub fn cancel_reward_alpha(ctx: Context<CancelRewardAlpha>, _bump_auth: u8, _bump_pot: u8) -> Result<()> {
        msg!("cancel reward alpha");
        instructions::cancel_reward_alpha::handler(ctx)
    }

    pub fn lock_reward(ctx: Context<LockReward>) -> Result<()> {
        msg!("lock reward");
        instructions::lock_reward::handler(ctx)
    }

    // --------------------------------------- rarities

    pub fn add_rarities_to_bank<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AddRaritiesToBank<'info>>,
        _bump_auth: u8,
        rarity_configs: Vec<RarityConfig>,
    ) -> Result<()> {
        msg!("add rarities to bank");
        instructions::add_rarities_to_bank::handler(ctx, rarity_configs)
    }

    pub fn create_alpha_tokenswap(ctx: Context<CreateAlphaTokenswap>, amount: u64) -> Result<()>{
        msg!("Create alpha tokenswap config");
        instructions::create_alpha_tokenswap::handler(ctx, amount)
    }

    pub fn transfer_alpha_tokens(ctx: Context<TransferAlphaTokens>, amount: u64) -> Result<()>{
        msg!("Transfer tokenswap");
        instructions::transfer_alpha_tokens::handler(ctx, amount)
    }
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn init_state(ctx: Context<InitState>, params: InitStateParams) -> Result<()> {
        InitState::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn update_result(ctx: Context<UpdateResult>) -> Result<()> {
        UpdateResult::actuate(&ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn request_result(ctx: Context<RequestResult>, params: RequestResultParams) -> Result<()> {
        RequestResult::actuate(&ctx, &params)
    }
}
