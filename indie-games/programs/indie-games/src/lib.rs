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

declare_id!("FohwxEdiTeT3ZY4r7rXH4dctCLTbA3S1pc8ibibHWaVa");

#[program]
pub mod indie_games {

    use super::*;

    pub fn initialize_dsc_vault(ctx: Context<InitializeDscTokenVaultContext>) -> Result<()> {
        initialize_dsc_vault_handler(ctx)
    }

    pub fn initialize_game(
        ctx: Context<InitializeGameContext>,
        args: InitializeGameArgs,
    ) -> Result<()> {
        initialize_game_handler(ctx, args)
    }

    pub fn initialize_assets(
        ctx: Context<InitializeAssetDataContext>,
        args: InitializeAssetDataArgs,
    ) -> Result<()> {
        intialize_asset_handler(ctx, args)
    }

    pub fn grant_asset_minting(
        ctx: Context<AddAssetAuthorityContext>,
        args: AddAssetAuthorityArgs,
    ) -> Result<()> {
        add_asset_authority_handler(ctx, args)
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

    pub fn initialize_assets_ata(
        ctx: Context<InitAssetATAContext>,
        args: InitAssetATAArgs,
    ) -> Result<()> {
        init_asset_ata_handler(ctx, args)
    }

    pub fn initialize_marketplace(ctx: Context<InitMarketplaceContext>) -> Result<()> {
        initialize_marketplace_handler(ctx)
    }

    pub fn list_asset(ctx: Context<ListForSaleContext>, args: ListAssetArgs) -> Result<()> {
        list_for_sale_handler(ctx, args)
    }

    pub fn buy_from_marketplace(ctx: Context<BuyFromSaleContext>) -> Result<()> {
        buy_from_sale_handler(ctx)
    }

    pub fn transfer_assets(
        ctx: Context<TransferAssetContext>,
        args: TransferAssetArgs,
    ) -> Result<()> {
        transfer_assets_handler(ctx, args)
    }
}
