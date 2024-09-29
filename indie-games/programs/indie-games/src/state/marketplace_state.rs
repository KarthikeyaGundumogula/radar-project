use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace{
    pub current_listing_id:u64,
}

#[account]
#[derive(InitSpace)]
pub struct Sale{
    pub listing_id:u64,
    pub price: u64,
    pub sale_state: u8,
    pub sale_amount:u64,
    pub dsc_credit_ata: Pubkey
}