use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

// use state::*;
// use errors::*;
use instructions::{
    asset_initialize_instructions::*, asset_management_instructions::*,
    asset_marketplace_instructions::*, game_instructions::*,
};

declare_id!("5aDMDM66ULuQvrjFtG4SnJT23mzoRfYmT4AnJZkPKgoe");

#[program]
pub mod indie_games {

    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGameContext>,
        args: InitializeGameArgs,
    ) -> Result<()> {
        initialize_game_handler(ctx, args)
    }

    pub fn initialize_assets(
        ctx: Context<InitializeAssetData>,
        args: InitializeAssetDataArgs,
    ) -> Result<()> {
        intialize_asset_handler(ctx, args)
    }

    pub fn mint_asset_as_owner(ctx: Context<MintAssetContext>, args: MintAssetArgs) -> Result<()> {
        mint_asset_handler(ctx, args)
    }

    pub fn mint_shared_asset(
        ctx: Context<MintAuthorizedAssetContext>,
        args: MintAuthorizedAssetArgs,
    ) -> Result<()> {
        mint_authorized_asset_handler(ctx, args)
    }

    pub fn list_asset(ctx: Context<ListForSaleContext>, args: ListAssetArgs) -> Result<()> {
        list_for_sale_handler(ctx, args)
    }

    pub fn buy_from_marketplace(
        ctx: Context<BuyFromSaleContext>,
    ) -> Result<()> {
        buy_from_sale_handler(ctx)
    }
}
