use crate::{
    errors::marketplace_errors::*, instructions::asset_management_instructions::*,
    state::marketplace_state::*,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Token, TokenAccount, Transfer as DscTransfer},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ListAssetArgs {
    pub asset_name: String,
    pub sale_price: u64,
    pub sale_amount: u64,
    pub market_authority: Pubkey,
    pub asset_game_id: Pubkey,
    pub dsc_credit_ata: Pubkey
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct BuyAssetArgs {
    pub asset_name: String,
    pub to_acc_authority: Pubkey,
    pub asset_game_id: Pubkey,
}

pub fn list_for_sale_handler(
    ctx: Context<TransferAssetContext>,
    args: ListAssetArgs,
) -> Result<()> {
    Ok(())
}

pub fn buy_from_sale_handler(ctx: Context<BuyFromSaleContext>, args: BuyAssetArgs) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: BuyAssetArgs)]
pub struct BuyFromSaleContext<'info> {
    #[account(mut)]
    pub from_dsc_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_dsc_ata: Account<'info,TokenAccount>,
    #[account(mut)]
    pub buyer_asset_ata: Account<'info,TokenAccount>,
    #[account(mut)]
    pub asset_holder_ata: Account<'info,TokenAccount>,
    #[account(mut)]
    pub asset_holder_authority: Account<'info,TokenAccount>,
    #[account(
        seeds = [&sale_acc.listing_id.to_le_bytes()],
        bump
    )]
    pub sale_acc: Option<Account<'info, Sale>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
