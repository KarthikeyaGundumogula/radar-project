use crate::{
    errors::marketplace_errors::*,
    instructions::asset_management_instructions::*,
    state::{asset_state::AssetAuthority, marketplace_state::*},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Token, TokenAccount, Transfer as SPLTransfer},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ListAssetArgs {
    pub asset_name: String,
    pub sale_price: u64,
    pub sale_amount: u64,
    pub market_authority: Pubkey,
    pub asset_game_id: Pubkey,
    pub dsc_credit_ata: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct BuyAssetArgs {
    pub asset_name: String,
    pub to_acc_authority: Pubkey,
    pub asset_game_id: Pubkey,
}

pub fn list_for_sale_handler(
    ctx: Context<TransferAssetContext>,
    args: ListAssetArgs,
) -> Result<()> {
    Ok(())
}

pub fn buy_from_sale_handler(ctx: Context<BuyFromSaleContext>, args: BuyAssetArgs) -> Result<()> {
    let sale_acc = &mut ctx.accounts.sale_acc;
    let dsc_cpi_accounts = SPLTransfer {
        from: ctx.accounts.buyer_dsc_ata.to_account_info(),
        to: ctx.accounts.seller_dsc_ata.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let dsc_cpi_ctx = CpiContext::new(cpi_program, dsc_cpi_accounts);
    transfer(dsc_cpi_ctx, sale_acc.price)?;

    let asset_cpi_accounts = SPLTransfer {
        from: ctx.accounts.asset_holding_ata.to_account_info(),
        to: ctx.accounts.buyer_asset_ata.to_account_info(),
        authority: ctx.accounts.asset_holding_ata_authority.to_account_info(),
    };
    let holding_ata = ctx.accounts.asset_holding_ata.key();
    let seeds: &[&[&[u8]]] = &[&[
        holding_ata.as_ref(),
        &[ctx.bumps.asset_holding_ata_authority],
    ]];
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let asset_cpi_ctx = CpiContext::new_with_signer(cpi_program, asset_cpi_accounts, seeds);
    transfer(asset_cpi_ctx, sale_acc.sale_amount)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: BuyAssetArgs)]
pub struct BuyFromSaleContext<'info> {
    #[account(mut)]
    pub buyer_dsc_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_dsc_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_asset_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub asset_holding_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [asset_holding_ata.key().as_ref()],
        bump
    )]
    pub asset_holding_ata_authority: AccountInfo<'info>,
    #[account(
        seeds = [&sale_acc.listing_id.to_le_bytes()],
        bump
    )]
    pub sale_acc: Account<'info, Sale>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
