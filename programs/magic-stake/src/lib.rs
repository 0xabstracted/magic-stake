use anchor_lang::prelude::*;
pub mod instructions;
pub mod number128;
pub mod state;

use gem_bank::instructions::record_rarity_points::RarityConfig;
use instructions::*;
use state::*;

declare_id!("45eAzw1V8BPznoTejeqtMvNP6suKDn9NWs4t5gRyK9TM");

#[program]
pub mod magic_stake {
    use super::*;
    pub fn init_fixed_farm(
        ctx: Context<InitFixedFarm>,
        bump_auth: u8,
        //reward_type_b: RewardType,
        farm_config: FarmConfig,
        max_counts: Option<MaxCounts>,
        farm_treasury_token: Pubkey,
    ) -> Result<()> {
        msg!("init fixed farm");
        instructions::init_fixed_farm::handler(
            ctx,
            bump_auth,
            farm_config,
            max_counts,
            farm_treasury_token
        )
    }
    pub fn init_probable_farm(
        ctx: Context<InitProbableFarm>,
        bump_auth: u8,
        _bump_treasury_token: u8,
        // lp_type: LPType,
        farm_config: FarmConfig,
        max_counts: Option<MaxCounts>,
    ) -> Result<()> {
        msg!("init farm");
        instructions::init_probable_farm::handler(
            ctx,
            bump_auth,
            // lp_type,
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
    pub fn init_fixed_farmer(ctx: Context<InitFixedFarmer>, index: u32) -> Result<()> {
        msg!("init farmer fixed");
        instructions::init_fixed_farmer::handler(ctx, index)
    }
    pub fn init_probable_farmer(ctx: Context<InitProbableFarmer>) -> Result<()> {
        msg!("init farmer probable");
        instructions::init_probable_farmer::handler(ctx)
    }
    
    pub fn stake(ctx: Context<Stake>, _bump_auth: u8, _bump_farmer: u8) -> Result<()> {
        msg!("stake");
        instructions::stake::handler(ctx)
    }
    
    pub fn unstake(
        ctx: Context<Unstake>,
        bump_auth: u8,
        _bump_treasury_token: u8,
        _bump_farmer: u8,
        bump_gem_box: u8,
        bump_gdr:u8,
        bump_rarity: u8,
        amount: u64,
        index: u32,
        skip_rewards: bool,
    ) -> Result<()> {
        msg!("unstake");
        instructions::unstake::handler(ctx, skip_rewards, bump_auth, bump_gem_box, bump_gdr, bump_rarity, amount, index)
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

    pub fn flash_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, FlashDeposit<'info>>,
        _bump_farmer: u8,
        bump_vault_auth: u8,
        bump_rarity: u8,
        index: u32,
        amount: u64,
    ) -> Result<()> {
        // msg!("flash deposit"); //have to remove all msgs! or run out of compute budget for this ix
        instructions::flash_deposit::handler(ctx, bump_vault_auth, bump_rarity, index, amount)
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
        // probable_rate_config: Option<ProbableRateConfig>,
        // lp_rate_config: Option<LPRateConfig>,
    ) -> Result<()> {
        msg!("fund reward");
        instructions::fund_reward::handler(
            ctx,
            /*variable_rate_config,*/ fixed_rate_config,
            // probable_rate_config,
            // lp_rate_config,
        )
    }

    pub fn cancel_reward(ctx: Context<CancelReward>, _bump_auth: u8, _bump_pot: u8) -> Result<()> {
        msg!("cancel reward");
        instructions::cancel_reward::handler(ctx)
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

    pub fn transfer_alpha_tokens(ctx: Context<TransferAlphaTokens>, bump_alpha_tokenswap:u8, bump_alpha_pot: u8, amount: u64) -> Result<()>{
        msg!("Transfer tokenswap");
        instructions::transfer_alpha_tokens::handler(ctx, amount, bump_alpha_tokenswap, bump_alpha_pot)
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
