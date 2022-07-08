use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use arrayref::array_ref;
use gem_common::TryAdd;

use crate::state::*;

#[derive(Accounts)]
pub struct AuthorizeFunder<'info> {
    #[account(mut, has_one = farm_manager)]
    pub farm: Box<Account<'info, Farm>>,
    #[account(mut)]
    pub farm_manager: Signer<'info>,
    ///CHECK:
    pub funder_to_authorize: AccountInfo<'info>,
    #[account(init_if_needed, seeds = [
            b"authorization".as_ref(),
            farm.key().as_ref(),
            funder_to_authorize.key().as_ref(),
        ],
        bump,
        payer = farm_manager,
        space = 8 + std::mem::size_of::<AuthorizationProof>()
    )]
    authorization_proof: Box<Account<'info, AuthorizationProof>>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AuthorizeFunder>) -> Result<()> {
    {
        let acct = ctx.accounts.authorization_proof.to_account_info();
        let data = &acct.try_borrow_data()?;
        let disc_bytes = array_ref![data, 0, 8];
        if disc_bytes != &AuthorizationProof::discriminator() && disc_bytes.iter().any(|a| a != &0)
        {
            return Err(error!(ErrorCode::AccountDiscriminatorMismatch));
        }
    }
    let proof = &mut ctx.accounts.authorization_proof;
    proof.authorized_funder = ctx.accounts.funder_to_authorize.key();
    proof.farm = ctx.accounts.farm.key();
    let farm = &mut ctx.accounts.farm;
    farm.authorized_funder_count.try_add_assign(1)?;
    msg!(
        "funder authorized: {}",
        ctx.accounts.funder_to_authorize.key()
    );
    Ok(())
}
