use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const APP_STATS_SEED: &[u8] = b"app-stats";
pub const PLAYER_SEED: &[u8] = b"player";
pub const FLAP_VAULT_SEED: &[u8] = b"flap-vault";
pub const AUTHORITY_SEED: &[u8] = b"authority";
pub const GAME_SEED: &[u8] = b"game";
pub const DENOMINATOR: u16 = 1000;
pub const PRIZE_PERCENT: u16 = 900;
pub const BURN_PERCENT: u16 = 10;
pub const WEEKLY_RAFFLE_PERCENT: u16 = 20;
pub const RANDOM_RAFFLE_PERCENT: u16 = 10;
pub const MONTHLY_RAFFLE_PERCENT: u16 = 20;
pub const COMMUNITY_PERCENT: u16 = 40;
pub const COMMUNITY_WALLET: Pubkey = pubkey!("Gaj7cGbQ3CCWkqn8QsnLXEVaBaTN98GRxkX1pPsC4yNS");
