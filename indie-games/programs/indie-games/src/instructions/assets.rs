use crate::errors::asset_errors::*;
use crate::state::asset_state::*;
use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeAssetDataArgs {
    game_id: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    price: u64,
    score: u8,
    trade_option: bool,
    collateral_option: bool,
    collateral: u64,
}

pub fn intialize_asset_handler(
    ctx: Context<InitializeAssetData>,
    args: InitializeAssetDataArgs,
) -> Result<()> {
    let asset = &mut ctx.accounts.asset_account;
    asset.game = args.game_id;
    require!(args.name.len() < 20, AssetErrors::InvalidArguments);
    asset.name = args.name;
    require!(args.symbol.len() < 5, AssetErrors::InvalidArguments);
    asset.symbol = args.symbol;
    require!(args.uri.len() < 20, AssetErrors::InvalidArguments);
    asset.uri = args.uri;
    asset.price = args.price;
    asset.score = args.score;
    asset.trade = args.trade_option;
    asset.collateral_option = args.collateral_option;
    asset.collateral = args.collateral;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args:InitializeAssetDataArgs)]
pub struct InitializeAssetData<'info> {
    #[account(
        init_if_needed,
        payer = creator,
        seeds=[args.name.as_bytes(),args.game_id.key().as_ref()],
        bump,
        space = 8 + AssetData::INIT_SPACE
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(
        init,
        seeds = [args.game_id.key().as_ref(),asset_account.key().as_ref()],
        bump,
        payer = creator,
        mint::decimals = 0,
        mint::authority = asset_account,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
