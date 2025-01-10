use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use switchboard_on_demand::RandomnessAccountData;

use crate::{
    error::ErrorCode, AppStats, Raffle, RaffleInfo, APP_STATS_SEED, AUTHORITY_SEED, RAFFLE_SEED,
    STABLE_VAULT_SEED,
};

#[derive(Accounts)]
pub struct RevealWinner<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub raffle: AccountLoader<'info, Raffle>,

    #[account(
        mut,
        seeds = [RAFFLE_SEED, raffle.key().as_ref()],
        bump
    )]
    pub raffle_info: Box<Account<'info, RaffleInfo>>,

    #[account(
        mut,
        seeds = [APP_STATS_SEED],
        bump,
        has_one = admin
    )]
    pub app_stats: Box<Account<'info, AppStats>>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    pub stable_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = admin,
        token::mint = stable_mint,
        token::authority = admin,
    )]
    pub stable_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [AUTHORITY_SEED],
        bump,
    )]
    /// CHECK:
    pub pda: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = stable_mint,
        token::authority = pda,
        seeds = [STABLE_VAULT_SEED, stable_mint.key().as_ref()],
        bump
    )]
    pub stable_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> RevealWinner<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.stable_vault.to_account_info().clone(),
            to: self.stable_account.to_account_info().clone(),
            authority: self.pda.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn reveal_winner_handler(ctx: Context<RevealWinner>) -> Result<()> {
    let bump: &[u8; 1] = &[ctx.bumps.pda];
    let seeds: &[&[u8]] = &[AUTHORITY_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    token::transfer(
        ctx.accounts.transfer_context().with_signer(signer_seeds),
        ctx.accounts.raffle_info.total_collect,
    )?;
    let clock: Clock = Clock::get()?;
    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    if randomness_data.seed_slot != ctx.accounts.raffle_info.commit_slot {
        return err!(ErrorCode::RandomnessExpired);
    }
    // call the switchboard on-demand get_value function to get the revealed random value
    let revealed_random_value: [u8; 32] = randomness_data
        .get_value(&clock)
        .map_err(|_| ErrorCode::RandomnessNotResolved)?;
    let raffle: &mut std::cell::RefMut<'_, Raffle> = &mut ctx.accounts.raffle.load_init()?;
    let rand_idx = u16::from_le_bytes([revealed_random_value[0], revealed_random_value[1]])
        % raffle.ticket_cnt;
    msg!("Rand: {:?}", rand_idx);
    let raffle_info: &mut Box<Account<'_, RaffleInfo>> = &mut ctx.accounts.raffle_info;
    raffle_info.winner = raffle.entrants[rand_idx as usize];
    Ok(())
}
