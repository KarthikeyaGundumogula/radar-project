use crate::{
    errors::marketplace_errors::*,
    state::{asset_state::*, marketplace_state::*},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer as SPLTransfer};

pub fn initialize_marketplace_handler(ctx: Context<InitMarketplaceContext>) -> Result<()> {
    let marketplace = &mut ctx.accounts.marketplace;
    marketplace.current_listing_id = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitMarketplaceContext<'info> {
    #[account(
        init,
        seeds = [b"marketplace"],
        bump,
        space = 8 + Marketplace::INIT_SPACE,
        payer = initializer
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ListAssetArgs {
    pub asset_name: String,
    pub sale_price: u64,
    pub sale_amount: u64,
    pub asset_game_id: Pubkey,
    pub dsc_credit_ata: Pubkey,
    pub asset_mint: Pubkey,
}

pub fn list_for_sale_handler(ctx: Context<ListForSaleContext>, args: ListAssetArgs) -> Result<()> {
    let asset_account = &ctx.accounts.asset_account;
    require!(asset_account.trade == true, MarketplaceError::CantListAsset);
    let market = &mut ctx.accounts.marketplace;
    let sale_acc = &mut ctx.accounts.sale_acc;
    sale_acc.listing_id = market.current_listing_id;
    sale_acc.price = args.sale_price;
    sale_acc.sale_amount = args.sale_amount;
    sale_acc.dsc_credit_ata = args.dsc_credit_ata;
    sale_acc.sale_state = 0;
    market.current_listing_id = market.current_listing_id.checked_add(1).unwrap();
    let seller_ata = &ctx.accounts.seller_asset_ata;
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let ata_key = seller_ata.key();
    require!(
        ata_key == ctx.accounts.seller.key(),
        MarketplaceError::NotAuthorized
    );
    let cpi_accounts = SPLTransfer {
        from: seller_ata.to_account_info(),
        to: ctx.accounts.market_asset_ata.to_account_info(),
        authority: ctx.accounts.seller_asset_ata_authority.to_account_info(),
    };
    let seeds: &[&[&[u8]]] = &[&[ata_key.as_ref()]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
    transfer(cpi_ctx, args.sale_amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: ListAssetArgs)]
pub struct ListForSaleContext<'info> {
    #[account(mut)]
    pub market_asset_ata: Account<'info, TokenAccount>,
    #[account(
        seeds=[args.asset_name.as_bytes(),args.asset_game_id.key().as_ref()],
        bump,
    )]
    pub asset_account: Account<'info, AssetData>,
    #[account(mut)]
    pub seller_asset_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_dsc_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [seller.key().as_ref(),args.asset_mint.as_ref()],
        bump
    )]
    pub seller_asset_ata_authority: Account<'info, AssetAuthority>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mut,
        seeds = [b"market_place"],
        bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        seeds = [marketplace.current_listing_id.to_string().as_bytes()],
        bump,
        payer = seller,
        space = 8 + Sale::INIT_SPACE
    )]
    pub sale_acc: Account<'info, Sale>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn buy_from_sale_handler(ctx: Context<BuyFromSaleContext>) -> Result<()> {
    let sale_acc = &mut ctx.accounts.sale_acc;
    require!(sale_acc.sale_state == 0, MarketplaceError::SaleNotFound);
    sale_acc.sale_state = 1;
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
pub struct BuyFromSaleContext<'info> {
    #[account(mut)]
    pub buyer_dsc_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_dsc_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_asset_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub asset_holding_ata: Account<'info, TokenAccount>,
    /// CHECK: unsafe
    #[account(
        seeds = [asset_holding_ata.key().as_ref()],
        bump
    )]
    pub asset_holding_ata_authority: AccountInfo<'info>,
    #[account(
        seeds = [&sale_acc.listing_id.to_string().as_bytes()],
        bump
    )]
    pub sale_acc: Account<'info, Sale>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
