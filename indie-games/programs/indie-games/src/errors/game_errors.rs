use anchor_lang::prelude::*;

#[error_code]
pub enum GameErrors {
    #[msg("Calle is not the owner")]
    UnAuthorizedOperation,
    #[msg("args check failed")]
    InvalidArgs
}