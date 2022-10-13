use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Registry {
    pub admin: Pubkey,
    pub vault_token_in: Pubkey,
    pub vault_token_out: Pubkey,
    pub rate_token_in: u64,
    pub rate_token_out: u64,
    pub mint_token_in: Pubkey,
    pub mint_token_out: Pubkey,
}
