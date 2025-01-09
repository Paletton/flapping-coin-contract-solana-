use anchor_lang::prelude::*;

#[account]
pub struct AppStats {
    pub admin: Pubkey,
    pub is_initialized: bool,
    pub flap_mint: Pubkey,
    pub total_amount: u64,
    pub player_count: u64,
}