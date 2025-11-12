use anchor_lang::prelude::*;

#[error_code]
pub enum FractionalizationError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("Not enough tokens to recalimNFT")]
    InsufficientTokens,
}
