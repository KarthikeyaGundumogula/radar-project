use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

// use state::*;
// use errors::*;
use instructions::assets::*;

declare_id!("tsiexNtYPMB9DJWKocWuC6Rx4DXgxjNkvuyCNiZdCYS");

#[program]
pub mod indie_games {
    use super::*;

    pub fn initialize_assets(
        ctx: Context<InitializeAssetData>,
        args: InitializeAssetDataArgs,
    ) -> Result<()> {
        intialize_asset_handler(ctx, args)
    }
}
