use anchor_lang::prelude::*;
use crate::error::*;

pub fn assert_authority(expected: &Pubkey, actual: &Pubkey) -> Result<()> {
    require_keys_eq!(*expected, *actual, FractionalizationError::Unauthorized);
    Ok(())
}