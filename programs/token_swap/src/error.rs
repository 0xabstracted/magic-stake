use anchor_lang::prelude::*;

#[error_code]
pub enum DispenserError {
    #[msg("Insufficient user funds")]
    InsufficientUserFunds,
    #[msg("Insufficient vault funds")]
    InsufficientVaultFunds,
    #[msg("Invalid calculation")]
    InvalidCalculation,
}
