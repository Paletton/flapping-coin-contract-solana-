pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5LeA7rgKeS5Mkga4Eju2pom9qbqs7WwUdfBs9zRvLYov");

#[program]
pub mod flap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit_handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw_handler(ctx, amount)
    }

    pub fn create_game(ctx: Context<CreateGame>, bet_amount: u64) -> Result<()> {
        create_game_handler(ctx, bet_amount)
    }

    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        join_game_handler(ctx)
    }

    pub fn settle_flip(ctx: Context<SettleFlip>) -> Result<()> {
        settle_flip_handler(ctx)
    }

    pub fn create_raffle(
        ctx: Context<CreateRaffle>,
        prize_amount: u64,
        ticket_price: u64,
        timestamp_start: i64,
        timestamp_end: i64,
        ticket_max: u16,
        raffle_type: u8,
    ) -> Result<()> {
        create_raffle_handler(
            ctx,
            prize_amount,
            ticket_price,
            timestamp_start,
            timestamp_end,
            ticket_max,
            raffle_type,
        )
    }

    pub fn buy_ticket (ctx: Context<BuyTicket>, ticket_demand: u16) -> Result<()> {
        buy_ticket_handler(ctx, ticket_demand)
    }

    pub fn reveal_winner(
        ctx: Context<RevealWinner>,
    ) -> Result<()> {
        reveal_winner_handler(ctx)
    }

    pub fn claime_prize(
        ctx: Context<ClaimPrize>,
    ) -> Result<()> {
        claime_prize_handler(ctx)
    }
}
