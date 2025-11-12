use anchor_lang::prelude::*;

#[derive(AnchorSerial, AnchorDeserailize, Clone)]
pub struct InitializeFractionalizationArgs{
    pub total_supply: u64,
}