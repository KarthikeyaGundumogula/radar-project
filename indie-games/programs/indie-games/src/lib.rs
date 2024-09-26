use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

// use state::*;
// use errors::*;
use instructions::{asset_initialize_instructions::*,game_instructions::*};

declare_id!("5aDMDM66ULuQvrjFtG4SnJT23mzoRfYmT4AnJZkPKgoe");

#[program]
pub mod indie_games {

    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGameContext>,
        args: InitializeGameArgs,
    ) -> Result<()> {
        initialize_game_handler(ctx,args)
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
}
