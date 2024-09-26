use crate::{
    errors::game_errors::*,
    state::{asset_state::*, game_state::*},
};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeGameArgs {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
}

pub fn initialize_game_handler(
    ctx: Context<InitializeGameContext>,
    args: InitializeGameArgs,
) -> Result<()> {
    let game_acc = &mut ctx.accounts.game_account;
    game_acc.name = args.name;
    game_acc.owner = args.owner;
    game_acc.description = args.description;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: InitializeGameArgs)]
pub struct InitializeGameContext<'info> {
    #[account(
        init,
        seeds = [args.owner.as_ref(),args.name.as_bytes()],
        payer = initializer,
        space = 8 + GameState::INIT_SPACE,
        bump,
    )]
    pub game_account: Account<'info, GameState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddAssetAuthorityArgs {
    pub source_game_name: String,
    pub source_game_id: Pubkey,
    pub to_game_id: Pubkey,
}

pub fn add_asset_authority(
    ctx: Context<AddAssetAuthorityContext>,
    args: AddAssetAuthorityArgs,
) -> Result<()> {
    let asset_account = &ctx.accounts.asset_account;
    let game_account = &ctx.accounts.game_account;
    let callee = ctx.accounts.game_owner.key();
    require!(
        asset_account.game == args.source_game_id,
        GameErrors::InvalidArgs
    );
    require!(
        game_account.owner == callee,
        GameErrors::UnAuthorizedOperation
    );
    let asset_authority_acc = &mut ctx.accounts.asset_authority_account;
    asset_authority_acc.user = args.to_game_id;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: AddAssetAuthorityArgs)]
pub struct AddAssetAuthorityContext<'info> {
    #[account(
        seeds = [game_owner.key().as_ref(),args.source_game_name.as_bytes()],
        bump
    )]
    pub game_account: Account<'info, GameState>,
    #[account(
        seeds = [args.source_game_name.as_bytes(),args.source_game_id.key().as_ref()],
        bump
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(
        init,
        seeds = [asset_account.key().as_ref(),game_account.key().as_ref(),args.to_game_id.as_ref()],
        payer = game_owner,
        space = 8+ AssetAuthority::INIT_SPACE,
        bump
    )]
    pub asset_authority_account: Account<'info, AssetAuthority>,
    #[account(mut)]
    pub game_owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
