use anchor_lang::prelude::*;

#[error_code]
pub enum AssetErrors {
    #[msg("Arguments dodn't matched")]
    InvalidArguments,
    #[msg("Minting new token failed")]
    MintFailed,
    #[msg("Only Game Owner can Create new Assets")]
    InvalidOperation,
    #[msg("Given Game Account and Asset Account are not related")]
    InvalidGameOrAssetAccount,
    #[msg("Cannot transfer non-tradable Assets")]
    InvalidTransfer
}