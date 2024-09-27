use crate::{
    errors::asset_errors::*,
    state::asset_state::*,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer as SplTransfer},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferAssetArgs {
    pub asset_name: String,
    pub amount: u64,
    pub asset_game_id: Pubkey,
    pub asset_account_id: Pubkey,
}

pub fn transfer_assets(ctx: Context<TransferAssetContext>, args: TransferAssetArgs) -> Result<()> {
    let from_acc = &ctx.accounts.from_ata;
    let from_acc_authority = &ctx.accounts.from_ata_authority;
    let to_acc = &ctx.accounts.to_ata;
    let token_program = &ctx.accounts.token_program;
    let asset_acc = &ctx.accounts.asset_account;
    require!(asset_acc.trade == true, AssetErrors::InvalidTransfer);
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
        mut,
        seeds = [args.asset_game_id.key().as_ref(),args.asset_account_id.as_ref()],
        bump
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(
        seeds = [args.asset_name.as_bytes(),args.asset_game_id.key().as_ref()],
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
    #[account(
        seeds = [from_ata.key().as_ref()],
        bump
    )]
    pub from_ata_authority: Account<'info, AssetAuthority>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
