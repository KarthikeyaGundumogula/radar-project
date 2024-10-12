use crate::errors::asset_errors::*;
use crate::state::{asset_state::*, game_state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer as DSC_Transfer},
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
    collateral_ratio: u64,
}

pub fn intialize_asset_handler(
    ctx: Context<InitializeAssetDataContext>,
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
    asset.collateral_ratio = args.collateral_ratio;
    msg!("Asset Data initialized along with mint account for the assets ");
    Ok(())
}

#[derive(Accounts)]
#[instruction(args:InitializeAssetDataArgs)]
pub struct InitializeAssetDataContext<'info> {
    #[account(
        init_if_needed,
        payer = creator,
        seeds=[args.name.as_bytes(),game_account.key().as_ref()],
        bump,
        space = 8 + AssetData::INIT_SPACE
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(
        init,
        seeds = [game_account.key().as_ref(),asset_account.key().as_ref()],
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
    pub amount: u64,
    pub asset_name: String,
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
    msg!("asset_authority step");
    msg!("token_program ...{}", ctx.accounts.token_program.key());
    let asset_authority = &mut ctx.accounts.destination_ata_authority;
    asset_authority.user = args.holder;
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = MintTo {
        authority: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.destination_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    msg!("before minting step");
    
    let asset_acc_key = asset_account.key();
    let game_acc_key = game_account.key();
    let seeds: &[&[&[u8]]] = &[&[
        game_acc_key.as_ref(),
        asset_acc_key.as_ref(),
        &[ctx.bumps.mint],
    ]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
    mint_to(cpi_ctx, args.amount)?;
    msg!("minted tokens {args.amount}");
    if asset_account.collateral_option == true {
        let collateral_ratio = asset_account.collateral_ratio.checked_div(100).unwrap();
        let price = asset_account.price;
        let collateral_factor = collateral_ratio.checked_mul(args.amount).unwrap();
        let collateral_deposit = collateral_factor.checked_mul(price).unwrap();
        let cpi_accounts = DSC_Transfer {
            from: ctx.accounts.user_dsc_token_ata.to_account_info(),
            to: ctx.accounts.collateral_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, collateral_deposit)?;
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(args:MintAssetArgs)]
pub struct MintAssetContext<'info> {
    #[account(
        mut,
        seeds=[game_account.key().as_ref(),asset_account.key().as_ref()],
        bump,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[args.asset_name.as_bytes(),game_account.key().as_ref()],
        bump,
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = destination_ata_authority,
    )]
    pub destination_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_dsc_token_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [args.holder.as_ref(),mint.key().as_ref()],
        bump,
        space = 8+AssetAuthority::INIT_SPACE
    )]
    pub destination_ata_authority: Account<'info, AssetAuthority>,
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
