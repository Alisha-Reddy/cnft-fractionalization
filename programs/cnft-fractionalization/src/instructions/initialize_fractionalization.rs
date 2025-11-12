use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Token};

use crate::state::*;
use crate::types::*;
use crate::constants::*;

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

    /// CHECK: This account will be initialized as a mint
    #[account(
        mut,
        signer,
    )]
    pub fraction_mint: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle(context: Context<InitializeFractionalization>, args: InitializeFractionalizationArgs) -> Result<()> {
    // Create the mint account
    let mint_size = 82; // Size of a Mint account
    let lamports = context.accounts.rent.minimum_balance(mint_size);
    
    anchor_lang::system_program::create_account(
        CpiContext::new(
            context.accounts.system_program.to_account_info(),
            anchor_lang::system_program::CreateAccount {
                from: context.accounts.owner.to_account_info(),
                to: context.accounts.fraction_mint.to_account_info(),
            },
        ),
        lamports,
        mint_size as u64,
        &context.accounts.token_program.key(),
    )?;

    // Initialize the mint with escrow as authority
    let escrow_key = context.accounts.escrow.key();
    
    token::initialize_mint(
        CpiContext::new(
            context.accounts.token_program.to_account_info(),
            InitializeMint {
                mint: context.accounts.fraction_mint.to_account_info(),
                rent: context.accounts.rent.to_account_info(),
            },
        ),
        0, // decimals
        &escrow_key, // mint authority
        Some(&escrow_key), // freeze authority
    )?;

    // Set escrow data
    let escrow = &mut context.accounts.escrow;
    escrow.mint = context.accounts.fraction_mint.key();
    escrow.authority = context.accounts.owner.key();
    escrow.total_supply = args.total_supply;
    escrow.bump = context.bumps.escrow;
    
    Ok(())
}
