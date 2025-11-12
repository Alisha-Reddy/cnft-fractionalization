use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

#[account]
pub struct EscrowAccount {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub total_supply: u64,
    pub bump: u8,
}

impl EscrowAccount {
    pub const LEN: usize =
        ANCHOR_DISCRIMINATOR_SIZE  // 8 bytes for Anchor discriminator
        + 32                       // authority Pubkey
        + 32                       // mint Pubkey
        + 8                        // total_supply (u64)
        + 1;                       // bump (u8)
}