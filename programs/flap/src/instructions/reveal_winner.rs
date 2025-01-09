use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{Raffle, RaffleInfo, RAFFLE_SEED, error::ErrorCode};

#[derive(Accounts)]
pub struct RevealWinner<'info> {
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

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
}

pub fn reveal_winner_handler(
    ctx: Context<RevealWinner>,
) -> Result<()> {
    let clock: Clock = Clock::get()?;
    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    if randomness_data.seed_slot != ctx.accounts.raffle_info.commit_slot {
        return err!(ErrorCode::RandomnessExpired);
    }
    // call the switchboard on-demand get_value function to get the revealed random value
    let revealed_random_value: [u8; 32] = randomness_data.get_value(&clock)
        .map_err(|_| ErrorCode::RandomnessNotResolved)?;
    let raffle: &mut std::cell::RefMut<'_, Raffle> = &mut ctx.accounts.raffle.load_init()?;
    let rand_idx = u16::from_le_bytes([revealed_random_value[0], revealed_random_value[1]]) % raffle.ticket_cnt;
    msg!("Rand: {:?}", rand_idx);
    let raffle_info: &mut Box<Account<'_, RaffleInfo>> = &mut ctx.accounts.raffle_info;
    raffle_info.winner = raffle.entrants[rand_idx as usize];
    Ok(())
}