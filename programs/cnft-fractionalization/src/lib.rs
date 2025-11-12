use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod types;
pub mod utils;

use instructions::*;
use crate::types::*;

declare_id!("FpWSGoETk8YmXu7bEQ1MEAu2nrzS6aU9x4tWkQFRh7XE");

#[program]
pub mod cnft_fractionalization {
    use super::*;

    pub fn initialize_fractionalization(
        ctx: Context<InitializeFractionalization>,
        args: InitializeFractionalizationArgs,
    ) -> Result<()>{
        initialize_fractionalization::handle(ctx, args)
    }

    pub fn reclaim(ctx: Context<Reclaim>) -> Result<()> {
        reclaim::handle(ctx)
    }
}
