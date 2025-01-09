use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{Game, Player, PLAYER_SEED, error::ErrorCode};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = game.is_ready == true,
    )]
    pub game: Box<Account<'info, Game>>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, signer.key().as_ref()],
        bump,
        constraint = player.owner == signer.key()
    )]
    pub player: Box<Account<'info, Player>>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
}

pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
    let game: &mut Box<Account<'_, Game>> = &mut ctx.accounts.game;
    let player: &mut Box<Account<'_, Player>> = &mut ctx.accounts.player;
    if !game.is_active {
        return err!(ErrorCode::GameIsNotActive);
    }
    if player.balance < game.bet_amount {
        return err!(ErrorCode::InsufficientBalance);
    }
    let randomness_data: std::cell::Ref<'_, RandomnessAccountData> = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    let clock: Clock = Clock::get()?;
    if randomness_data.seed_slot != clock.slot - 1 {
        msg!("seed_slot: {}", randomness_data.seed_slot);
        msg!("slot: {}", clock.slot);
        return err!(ErrorCode::RandomnessAlreadyRevealed);
    }
    player.balance -= game.bet_amount;
    game.player_right = ctx.accounts.signer.key();
    // decide winner from vr
    game.commit_slot = randomness_data.seed_slot;
    game.randomness_account = ctx.accounts.randomness_account_data.key();
    // Log the result
    msg!("Coin flip initiated, randomness requested.");
    Ok(())
}