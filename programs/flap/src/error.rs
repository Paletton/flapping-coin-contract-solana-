use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid player")]
    InvalidPlayer,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Game is active")]
    GameIsActive,
    #[msg("Game is not active")]
    GameIsNotActive,
    #[msg("Randomness was already revealed")]
    RandomnessAlreadyRevealed,
}
