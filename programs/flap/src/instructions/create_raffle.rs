use anchor_lang::prelude::*;

use crate::{Raffle, RaffleInfo, RAFFLE_SEED};
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
    Ok(())
}