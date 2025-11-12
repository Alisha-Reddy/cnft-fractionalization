use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeFractionalizationArgs{
    pub total_supply: u64,
}