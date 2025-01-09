use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{AppStats, Raffle, RaffleInfo, APP_STATS_SEED, RAFFLE_SEED, error::ErrorCode};
use std::mem::size_of;

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(mut)]
    pub raffle_creator: Signer<'info>,

    #[account(
        init,
        payer = raffle_creator,
        space = size_of::<Raffle>() + 8,
    )]
    pub raffle: AccountLoader<'info, Raffle>,

    #[account(
        init,
        payer = raffle_creator,
        space = size_of::<RaffleInfo>() + 8,
        seeds = [RAFFLE_SEED, raffle.key().as_ref()],
        bump
    )]
    pub raffle_info: Box<Account<'info, RaffleInfo>>,

    #[account(
        mut,
        seeds = [APP_STATS_SEED],
        bump,
        constraint = app_stats.admin == raffle_creator.key(),
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_raffle_handler(
    ctx: Context<CreateRaffle>,
    prize_amount: u64,
    ticket_price: u64,
    timestamp_start: i64,
    timestamp_end: i64,
    ticket_max: u16,
    raffle_type: u8
) -> Result<()> {
    let raffle: &mut std::cell::RefMut<'_, Raffle> = &mut ctx.accounts.raffle.load_init()?;
    let raffle_info: &mut Box<Account<'_, RaffleInfo>> = &mut ctx.accounts.raffle_info;
    raffle.ticket_cnt = 0;
    raffle.ticket_max = ticket_max;
    raffle.cnt_entrants = 0;
    raffle_info.ticket_price = ticket_price;
    raffle_info.prize_amount = prize_amount;
    raffle_info.timestamp_start = timestamp_start;
    raffle_info.timestampe_end = timestamp_end;
    raffle_info.winner = Pubkey::default();
    raffle_info.raffle_type = raffle_type;
    raffle_info.claimed = false;
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    if raffle_type == 0 {
        if prize_amount > app_stats.weekly_raffle_amount {
            return err!(ErrorCode::InvalidAmount);
        }
        app_stats.weekly_raffle_amount -= prize_amount;
    } else if raffle_type == 1 {
        if prize_amount > app_stats.monthly_raffle_amount {
            return err!(ErrorCode::InvalidAmount);
        }
        app_stats.monthly_raffle_amount -= prize_amount;
    } else if raffle_type == 2 {
        if prize_amount > app_stats.random_raffle_amount {
            return err!(ErrorCode::InvalidAmount);
        }
        app_stats.random_raffle_amount -= prize_amount;
    } else {
        return err!(ErrorCode::InvalidType);
    }
    let randomness_data: std::cell::Ref<'_, RandomnessAccountData> = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    let clock: Clock = Clock::get()?;
    if randomness_data.seed_slot != clock.slot - 1 {
        msg!("seed_slot: {}", randomness_data.seed_slot);
        msg!("slot: {}", clock.slot);
        return err!(ErrorCode::RandomnessAlreadyRevealed);
    }
    raffle_info.randomness_account = ctx.accounts.randomness_account_data.key();
    raffle_info.commit_slot = randomness_data.seed_slot;
    Ok(())
}