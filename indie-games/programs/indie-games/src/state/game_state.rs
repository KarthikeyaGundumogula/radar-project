use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameState {
    pub owner: Pubkey,
    #[max_len(10)]
    pub name: String,
    #[max_len(50)]
    pub description: String
}