use crate::errors::asset_errors::*;
use crate::state::{asset_state::*, game_state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
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
    let signer = &ctx.accounts.creator;
    require!(
        signer.key() == ctx.accounts.game_account.owner,
        AssetErrors::InvalidOperation
    );
    asset.price = args.price;
    asset.score = args.score;
    asset.trade = args.trade_option;
    asset.collateral_option = args.collateral_option;
    asset.collateral = args.collateral;
    msg!("Asset Data initialized along with mint account for the assets ");
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
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds = [creator.key().as_ref(),game_account.name.as_bytes()],
        bump
    )]
    pub game_account: Account<'info, GameState>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintAssetArgs {
    pub game_id: Pubkey,
    pub amount: u64,
    pub name: String,
    pub game_name: String,
    pub holder: Pubkey,
}

pub fn mint_asset_handler(ctx: Context<MintAssetContext>, args: MintAssetArgs) -> Result<()> {
    let signer = &ctx.accounts.user;
    let game_account = &ctx.accounts.game_account;
    let asset_account = &ctx.accounts.asset_account;
    require!(
        signer.key() == game_account.owner,
        AssetErrors::InvalidOperation,
    );
    require!(
        game_account.key() == asset_account.game,
        AssetErrors::InvalidGameOrAssetAccount
    );
    let asset_authority = &mut ctx.accounts.asset_authority;
    asset_authority.user = args.holder;
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = MintTo {
        authority: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    let binding = ctx.accounts.asset_account.key();
    let seeds: &[&[&[u8]]] = &[&[args.game_id.as_ref(), binding.as_ref(), &[ctx.bumps.mint]]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);

    mint_to(cpi_ctx, args.amount)?;
    msg!("minted tokens {args.amount}");
    Ok(())
}

#[derive(Accounts)]
#[instruction(args:MintAssetArgs)]
pub struct MintAssetContext<'info> {
    #[account(
        mut,
        seeds=[game_account.key().as_ref(),asset_account.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[args.name.as_bytes(),args.game_id.key().as_ref()],
        bump,
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = asset_authority,
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [token_account.key().as_ref()],
        bump,
        space = 8+AssetAuthority::INIT_SPACE
    )]
    pub asset_authority: Account<'info, AssetAuthority>,
    #[account(
        seeds = [user.key().as_ref(),args.game_name.as_bytes()],
        bump
    )]
    pub game_account: Account<'info, GameState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
