use anchor_lang::prelude::*;

use crate::{Raffle, RaffleInfo, RAFFLE_SEED, error::ErrorCode};

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub raffle: AccountLoader<'info, Raffle>,

    #[account(
        mut,
        seeds = [RAFFLE_SEED, raffle.key().as_ref()],
        bump
    )]
    pub raffle_info: Box<Account<'info, RaffleInfo>>,
}

pub fn buy_ticket_handler(ctx: Context<BuyTicket>, ticket_demand: u16) -> Result<()> {
    let timestamp: i64 = Clock::get()?.unix_timestamp;
    let mut raffle: std::cell::RefMut<'_, Raffle> = ctx.accounts.raffle.load_mut()?;
    let raffle_info: &mut Box<Account<'_, RaffleInfo>> = &mut ctx.accounts.raffle_info;
    let buyer: Pubkey = ctx.accounts.signer.key();
    if timestamp < raffle_info.timestamp_start {
        return err!(ErrorCode::RaffleNotStarted);
    }
    if timestamp > raffle_info.timestampe_end {
        return err!(ErrorCode::RaffleEnded);
    }
    if raffle.ticket_max < raffle.ticket_cnt + ticket_demand {
        return err!(ErrorCode::NotEnoughTicketLeft);
    }
    raffle.append(buyer, ticket_demand);
    Ok(())
}
