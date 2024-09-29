use crate::{
    errors::asset_errors::AssetErrors,
    state::{asset_state::*, game_state::*},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer as SplTransfer},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintAuthorizedAssetArgs {
    pub source_game_id: Pubkey,
    pub asset_account_id: Pubkey,
    pub amount: u64,
    pub name: String,
    pub holder: Pubkey,
    pub game_owner: Pubkey,
    pub game_name: String,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferAssetArgs {
    pub asset_name: String,
    pub amount: u64,
    pub to_account_authority: Pubkey,
    pub asset_game_id: Pubkey,
    pub sale_id: u64,
}

pub fn mint_authorized_asset_handler(
    ctx: Context<MintAuthorizedAssetContext>,
    args: MintAuthorizedAssetArgs,
) -> Result<()> {
    let signer = &ctx.accounts.user;
    let mint_authority = &ctx.accounts.mint_authority;
    let asset_account = &ctx.accounts.asset_account;
    let game = ctx.accounts.game_account.key();
    require!(
        signer.key() == mint_authority.user,
        AssetErrors::InvalidOperation
    );
    require!(
        game == asset_account.game,
        AssetErrors::InvalidGameOrAssetAccount
    );
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = MintTo {
        authority: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    let binding = ctx.accounts.asset_account.key();
    let seeds: &[&[&[u8]]] = &[&[game.as_ref(), binding.as_ref(), &[ctx.bumps.mint]]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);

    mint_to(cpi_ctx, args.amount)?;
    msg!("minted tokens {args.amount}");
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: MintAuthorizedAssetArgs)]
pub struct MintAuthorizedAssetContext<'info> {
    #[account(
        mut,
        seeds=[game_account.key().as_ref(),asset_account.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[args.name.as_bytes(),game_account.key().as_ref()],
        bump
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(mut)]
    pub token_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [args.game_owner.as_ref(),args.game_name.as_bytes()],
        bump
    )]
    pub game_account: Account<'info, GameState>,
    #[account(
        seeds = [asset_account.key().as_ref(),game_account.key().as_ref(),user.key().as_ref()],
        bump
    )]
    pub mint_authority: Account<'info, MintAuthority>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn transfer_assets(ctx: Context<TransferAssetContext>, args: TransferAssetArgs) -> Result<()> {
    let from_acc = &ctx.accounts.from_ata;
    let from_acc_authority = &ctx.accounts.from_ata_authority;
    let to_acc = &ctx.accounts.to_ata;
    let token_program = &ctx.accounts.token_program;
    let asset_acc = &ctx.accounts.asset_account;
    let signer = &ctx.accounts.user;
    require!(asset_acc.trade == true, AssetErrors::InvalidTransfer);
    require!(
        from_acc_authority.user == signer.key(),
        AssetErrors::InvalidOperation
    );
    let cpi_accounts = SplTransfer {
        from: from_acc.to_account_info().clone(),
        to: to_acc.to_account_info().clone(),
        authority: from_acc_authority.to_account_info().clone(),
    };

    let cpi_program = token_program.to_account_info();
    let from = from_acc.key();
    let seeds: &[&[&[u8]]] = &[&[from.as_ref(), &[ctx.bumps.from_ata_authority]]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
    transfer(cpi_ctx, args.amount)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: TransferAssetArgs)]
pub struct TransferAssetContext<'info> {
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [from_ata.key().as_ref()],
        bump
    )]
    pub from_ata_authority: Account<'info, AssetAuthority>,
    #[account(
        seeds = [args.asset_name.as_bytes(),args.asset_game_id.as_ref()],
        bump
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitAssetATAArgs {
    pub game_id: Pubkey,
    pub asset_name: String,
}

pub fn init_asset_ata(ctx: Context<InitAssetATAContext>, _args: InitAssetATAArgs) -> Result<()> {
    let ata_authority = &mut ctx.accounts.to_ata_authority;
    ata_authority.user = ctx.accounts.user.key();
    msg!("asset account initialized");
    Ok(())
}

#[derive(Accounts)]
#[instruction(_args: InitAssetATAArgs)]
pub struct InitAssetATAContext<'info> {
    #[account(
        mut,
        seeds = [_args.game_id.as_ref(),asset_account.key().as_ref()],
        bump
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(
        seeds = [_args.asset_name.as_bytes(),_args.game_id.as_ref()],
        bump
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_account,
        associated_token::authority = to_ata_authority,
    )]
    pub to_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [to_ata.key().as_ref()],
        bump,
        space = 8+AssetAuthority::INIT_SPACE
    )]
    pub to_ata_authority: Account<'info, AssetAuthority>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
