use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

pub fn handle(ctx: Context<Reclaim>) -> Result<()> {
    // TODO: burn logic, transfer cNFT, etc.
    Ok(())
}

#[derive(Accounts)]
pub struct Reclaim<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub escrow: Account<'info, EscrowAccount>
}