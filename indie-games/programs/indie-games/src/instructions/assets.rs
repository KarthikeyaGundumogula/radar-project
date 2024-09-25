use crate::errors::asset_errors::*;
use crate::state::asset_state::*;
use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
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
    asset.price = args.price;
    asset.score = args.score;
    asset.trade = args.trade_option;
    asset.collateral_option = args.collateral_option;
    asset.collateral = args.collateral;
    msg!("Asset Data initialized along with mint account for the assets ");
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintAssetArgs {
    pub game_id: Pubkey,
    pub amount: u64,
    pub name: String,
}

pub fn mint_asset_handler(ctx: Context<MintAssetContext>, args: MintAssetArgs) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = MintTo {
        authority: ctx.accounts.asset_account.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    let seeds = vec![ctx.bumps.asset_account];
    let binding = args.game_id;
    let seeds = vec![args.name.as_bytes(), binding.as_ref(), seeds.as_slice()];
    let seeds = vec![seeds.as_slice()];
    let seeds = seeds.as_slice();
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
        seeds=[args.game_id.as_ref(),asset_account.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[args.name.as_bytes(),args.game_id.as_ref()],
        bump,
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        has_one = mint
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
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
