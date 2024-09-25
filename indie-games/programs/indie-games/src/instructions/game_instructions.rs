use crate::state::game_state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeGameArgs {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
}

pub fn initialize_game_handler(ctx:Context<InitializeGameContext>,args:InitializeGameArgs) -> Result<()>{
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
