use anchor_lang::prelude::*;

declare_id!("EAwKGvgAJeTMaMHF8UYwMGmXCWBp4NCjmta534nEAodG");

#[program]
pub mod indie_games_dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
