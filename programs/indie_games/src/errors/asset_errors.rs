use anchor_lang::prelude::*;

#[error_code]
pub enum AssetErrors {
    #[msg("Arguments dodn't matched")]
    InvalidArguments,
    #[msg("Minting new token failed")]
    MintFailed
}