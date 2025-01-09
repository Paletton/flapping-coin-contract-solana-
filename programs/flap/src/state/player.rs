use anchor_lang::prelude::*;

#[account]
pub struct Player {
    pub owner: Pubkey,
    pub balance: u64,
    pub initialized: bool,
}