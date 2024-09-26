use crate::{
    errors::asset_errors::AssetErrors,
    state::{asset_state::*, game_state::*},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
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
    let asset_authority = &mut ctx.accounts.asset_authority;
    asset_authority.user = args.holder;
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = MintTo {
        authority: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
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
        seeds=[args.source_game_id.key().as_ref(),asset_account.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[args.name.as_bytes(),game_account.key().as_ref()],
        bump
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
        seeds = [args.game_owner.as_ref(),args.game_name.as_bytes()],
        bump
    )]
    pub game_account: Account<'info, GameState>,
    #[account(
        seeds = [asset_account.key().as_ref(),game_account.key().as_ref(),user.key().as_ref()],
        bump
    )]
    pub mint_authority: Account<'info, MintAuthority>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [token_account.key().as_ref()],
        bump,
        space = 8+AssetAuthority::INIT_SPACE
    )]
    pub asset_authority: Account<'info, AssetAuthority>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
