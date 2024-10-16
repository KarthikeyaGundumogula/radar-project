use crate::{
    errors::game_errors::*,
    state::{asset_state::*, game_state::*},
};
use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token_interface};

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

pub fn initialize_dsc_vault_handler(_ctx: Context<InitializeDscTokenVaultContext>) -> Result<()> {
    msg!("Dsc token Vault Initialized");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeDscTokenVaultContext<'info> {
    // PDA, auth over all token vaults
    /// CHECK: unsafe
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    pub dsc_token_ata_authority: AccountInfo<'info>,
    #[account(
        init,
        token::mint = token_mint,
        token::authority = dsc_token_ata_authority,
        seeds = [b"token_vault"],
        bump,
        payer = initializer,
    )]
    pub dsc_token_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
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
pub struct GrantMintAuthorityArgs {
    pub source_game_name: String,
    pub source_game_id: Pubkey,
    pub to_game_id: Pubkey,
}

pub fn grant_mint_authority_handler(
    ctx: Context<GrantMintAuthorityContext>,
    args: GrantMintAuthorityArgs,
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
    let mint_auth_acc = &mut ctx.accounts.mint_authority_account;
    mint_auth_acc.user = args.to_game_id;
    mint_auth_acc.asset_account = asset_account.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: GrantMintAuthorityArgs)]
pub struct GrantMintAuthorityContext<'info> {
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
    pub mint_authority_account: Account<'info, MintAuthority>,
    #[account(mut)]
    pub game_owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
