use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Marketplace account is not initialized")]
    MarketplaceNotInitialized,
    #[msg("sale Ids may be Owerflown")]
    ArithmeticError,
    #[msg("sale acc not exists")]
    SaleNotFound,
    #[msg("Asset trding is disbled for this asset")]
    CantListAsset,
    #[msg("Seller is not the authorized of the ata")]
    NotAuthorized
}