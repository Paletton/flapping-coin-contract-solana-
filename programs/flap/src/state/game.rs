use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player_left: Pubkey,
    pub player_right: Pubkey,
    pub winner: Pubkey,
    pub is_active: bool,
    pub bet_amount: u64,
    pub is_ready: bool,
    pub randomness_account: Pubkey,
    pub commit_slot: u64,
}