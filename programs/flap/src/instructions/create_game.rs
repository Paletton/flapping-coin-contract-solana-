use anchor_lang::prelude::*;

use crate::{Game, Player, PLAYER_SEED, error::ErrorCode};
use std::mem::size_of;

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init_if_needed,
        payer = creator,
        space = size_of::<Game>() + 8,
    )]
    pub game: Box<Account<'info, Game>>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, creator.key().as_ref()],
        bump,
        constraint = player.owner == creator.key()
    )]
    pub player: Box<Account<'info, Player>>,

    pub system_program: Program<'info, System>,
}

pub fn create_game_handler(ctx: Context<CreateGame>, bet_amount: u64) -> Result<()> {
    let game: &mut Box<Account<'_, Game>> = &mut ctx.accounts.game;
    let player: &mut Box<Account<'_, Player>> = &mut ctx.accounts.player;
    if game.is_active {
        return err!(ErrorCode::GameIsActive);
    }
    if player.balance < bet_amount {
        return err!(ErrorCode::InsufficientBalance);
    }
    player.balance -= bet_amount;
    game.player_left = ctx.accounts.creator.key();
    game.player_right = Pubkey::default();
    game.winner = Pubkey::default();
    game.bet_amount = bet_amount;
    game.randomness_account = Pubkey::default();
    game.wager = 100;
    game.randomness_account = Pubkey::default();
    game.is_ready = true;
    game.is_active = true;
    Ok(())
}