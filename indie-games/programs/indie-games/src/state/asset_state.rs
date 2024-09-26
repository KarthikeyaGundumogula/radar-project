use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AssetData {
    pub game: Pubkey,
    #[max_len(20)]
    pub name: String,
    #[max_len(5)]
    pub symbol: String,
    #[max_len(20)]
    pub uri: String,
    pub price: u64,
    pub score: u8,
    pub trade: bool,
    pub collateral_option: bool,
    pub collateral: u64,
}

#[account]
#[derive(InitSpace)]
pub struct AssetAuthority {
    pub user: Pubkey
}

#[account]
#[derive(InitSpace)]
pub struct MintAuthority {
    pub user: Pubkey,
    pub asset_account: Pubkey,
}
