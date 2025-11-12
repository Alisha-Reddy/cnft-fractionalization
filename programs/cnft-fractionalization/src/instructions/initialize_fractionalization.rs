use std::alloc::System;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::*;
use crate::types::*;
use crate::constants::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct InitializeFractionalization<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = EscrowAccount::LEN,
        seeds = [SEED_ESCROW, fraction_mint.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint_authority = escrow,
        mint::freeze_authority = escrow
    )]
    pub fraction_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

pub fn handle(ctx: Context<InitializeFractionalization>, args: InitializeFractionalizationArgs) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.bump = *context.accounts.fraction_mint.key();
    escrow.mint = context.accounts.fraction_mint.key();
    escrow.authority = context.accounts.owner.key();
    escrow.total_supply = total_supply;
    
    Ok(())

}
