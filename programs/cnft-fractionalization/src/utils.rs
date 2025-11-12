use anchor_lang::prelude::*;
pub fn assert_authority(expected: &Pubkey, actual: &Pubkey) -> Result<()> {
    require_keys_eq!(*expected, *actual, crate::errors::FractionalizationError:Inauthorized);
    Ok(())
}