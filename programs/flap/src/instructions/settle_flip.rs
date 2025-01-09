use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use switchboard_on_demand::RandomnessAccountData;

use crate::{
    error::ErrorCode, AppStats, Game, Player, APP_STATS_SEED, AUTHORITY_SEED, BURN_PERCENT,
    COMMUNITY_PERCENT, COMMUNITY_WALLET, DENOMINATOR, MONTHLY_RAFFLE_PERCENT, PLAYER_SEED,
    PRIZE_PERCENT, RANDOM_RAFFLE_PERCENT, WEEKLY_RAFFLE_PERCENT,
};

#[derive(Accounts)]
pub struct SettleFlip<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = game.is_ready == true,
    )]
    pub game: Box<Account<'info, Game>>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    #[account(
        seeds = [AUTHORITY_SEED],
        bump,
    )]
    /// CHECK:
    pub pda: UncheckedAccount<'info>,

    #[account(
        mut,
        mint::authority = pda,
    )]
    pub flap_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::mint = flap_mint,
        token::authority = pda,
    )]
    pub flap_vault: Box<Account<'info, TokenAccount>>,

    #[account(address = COMMUNITY_WALLET)]
    pub community_wallet: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        token::mint = flap_mint,
        token::authority = community_wallet
    )]
    pub community_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, game.player_left.as_ref()],
        bump,
    )]
    pub player_left: Box<Account<'info, Player>>,

    #[account(
        mut,
        seeds = [PLAYER_SEED, game.player_right.as_ref()],
        bump,
    )]
    pub player_right: Box<Account<'info, Player>>,

    #[account(
        mut,
        seeds = [APP_STATS_SEED],
        bump,
        has_one = flap_mint
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SettleFlip<'info> {
    fn burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts: Burn = Burn {
            mint: self.flap_mint.to_account_info(),
            from: self.flap_vault.to_account_info(),
            authority: self.pda.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.flap_vault.to_account_info().clone(),
            to: self.community_token_account.to_account_info().clone(),
            authority: self.pda.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn settle_flip_handler(ctx: Context<SettleFlip>) -> Result<()> {
    let clock: Clock = Clock::get()?;
    // call the switchboard on-demand parse function to get the randomness data
    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    if randomness_data.seed_slot != ctx.accounts.game.commit_slot {
        return err!(ErrorCode::RandomnessExpired);
    }
    // call the switchboard on-demand get_value function to get the revealed random value
    let revealed_random_value: [u8; 32] = randomness_data
        .get_value(&clock)
        .map_err(|_| ErrorCode::RandomnessNotResolved)?;
    // Use the revealed random value to determine the flip results
    let randomness_result = revealed_random_value[0] % 2 == 0;
    if randomness_result {
        msg!("FLIP_RESULT: Heads");
    } else {
        msg!("FLIP_RESULT: Tails");
    }

    // burn token
    let burn_amount: u64 =
        ctx.accounts.game.bet_amount * 2 * BURN_PERCENT as u64 / DENOMINATOR as u64;
    let bump: &[u8; 1] = &[ctx.bumps.pda];
    let seeds: &[&[u8]] = &[AUTHORITY_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    token::burn(
        ctx.accounts.burn_context().with_signer(signer_seeds),
        burn_amount,
    )?;
    // tranfer to community wallet
    let community_amount: u64 =
        ctx.accounts.game.bet_amount * 2 * COMMUNITY_PERCENT as u64 / DENOMINATOR as u64;
    token::transfer(
        ctx.accounts.transfer_context().with_signer(signer_seeds),
        community_amount,
    )?;
    // format game account
    let game: &mut Box<Account<'_, Game>> = &mut ctx.accounts.game;
    let prize_amount: u64 = game.bet_amount * 2 * PRIZE_PERCENT as u64 / DENOMINATOR as u64;
    if randomness_result {
        game.winner = game.player_left;
        ctx.accounts.player_left.balance += prize_amount;
    } else {
        game.winner = game.player_right;
        ctx.accounts.player_right.balance += prize_amount;
    }
    game.is_active = false;
    game.is_ready = false;
    // update app stats
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    app_stats.total_amount -= burn_amount;
    app_stats.burnt_amount += burn_amount;
    app_stats.total_amount -= community_amount;
    let weekly_raffle_amount: u64 =
        game.bet_amount * 2 * WEEKLY_RAFFLE_PERCENT as u64 / DENOMINATOR as u64;
    app_stats.total_amount -= weekly_raffle_amount;
    app_stats.weekly_raffle_amount += weekly_raffle_amount;
    let randome_raffle_amount: u64 =
        game.bet_amount * 2 * RANDOM_RAFFLE_PERCENT as u64 / DENOMINATOR as u64;
    app_stats.total_amount -= randome_raffle_amount;
    app_stats.random_raffle_amount += randome_raffle_amount;
    let monthly_raffle_amount: u64 =
        game.bet_amount * 2 * MONTHLY_RAFFLE_PERCENT as u64 / DENOMINATOR as u64;
    app_stats.total_amount -= monthly_raffle_amount;
    app_stats.monthly_raffle_amount += monthly_raffle_amount;
    Ok(())
}
