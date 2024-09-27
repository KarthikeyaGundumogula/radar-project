use crate::{errors::marketplace_errors::*, instructions::asset_management_instructions::*};
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ListAssetArgs {
    pub asset_name: String,
    pub sale_price: u64,
    pub sale_amount: u64,
    pub market_authority: Pubkey,
    pub asset_game_id: Pubkey,
}

#[derive(AnchorDeserialize,AnchorSerialize)]
pub struct BuyAssetArgs {
    pub asset_name:String,
    pub to_acc_authority: Pubkey,
    pub asset_game_id: Pubkey
}

pub fn list_for_sale(ctx: Context<TransferAssetContext>, args: ListAssetArgs) -> Result<()> {
    let current_listing_id = if let Some(market_acc) = &mut ctx.accounts.market_place {
        market_acc.reload()?;
        let new_id = market_acc
            .current_listing_id
            .checked_add(1)
            .ok_or(error!(MarketplaceError::ArithmeticError))?;
        market_acc.current_listing_id = new_id;
        market_acc.reload()?;
        new_id
    } else {
        return Err(error!(MarketplaceError::MarketplaceNotInitialized));
    };

    if let Some(sale_acc) = &mut ctx.accounts.sale_acc {
        sale_acc.listing_id = current_listing_id;
        sale_acc.price = args.sale_price;
        sale_acc.sale_amount = args.sale_amount;
        sale_acc.sale_state = 0;
    }

    let transfer_args = TransferAssetArgs {
        asset_name: args.asset_name,
        amount: args.sale_amount,
        to_account_authority: args.market_authority,
        sale_id: current_listing_id,
        asset_game_id: args.asset_game_id,
    };

    transfer_assets(ctx, transfer_args)
}

pub fn buy_from_sale(ctx: Context<TransferAssetContext>,args: BuyAssetArgs) -> Result<()> {
    let sale = if let Some(sale_acc) = &mut ctx.accounts.sale_acc {
        sale_acc.sale_state = 1;
        sale_acc
    }
    else {
        return Err(error!(MarketplaceError::SaleNotFound));
    };

    let transfer_args= TransferAssetArgs{
        asset_name: args.asset_name,
        amount: sale.sale_amount,
        to_account_authority: args.to_acc_authority,
        sale_id: sale.listing_id,
        asset_game_id: args.asset_game_id
    };

    transfer_assets(ctx,transfer_args)
}
