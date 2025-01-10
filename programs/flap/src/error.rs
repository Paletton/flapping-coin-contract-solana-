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
    #[msg("Randomness was expired")]
    RandomnessExpired,
    #[msg("Randomness is not resolved")]
    RandomnessNotResolved,
    #[msg("Amount is invalid")]
    InvalidAmount,
    #[msg("Type is invalid")]
    InvalidType,
    #[msg("Raffle is not started")]
    RaffleNotStarted,
    #[msg("Raffle was ended")]
    RaffleEnded,
    #[msg("Left ticket is not enough")]
    NotEnoughTicketLeft,
    #[msg("You can buy only one time")]
    OnlyOneTime
}
