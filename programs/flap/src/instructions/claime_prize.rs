use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{Raffle, RaffleInfo, AUTHORITY_SEED, RAFFLE_SEED};

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,

    #[account(mut)]
    pub raffle: AccountLoader<'info, Raffle>,

    #[account(
        mut,
        seeds = [RAFFLE_SEED, raffle.key().as_ref()],
        bump,
        has_one = winner
    )]
    pub raffle_info: Box<Account<'info, RaffleInfo>>,

    pub flap_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = winner,
        token::mint = flap_mint,
        token::authority = winner,
    )]
    pub winner_flap_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [AUTHORITY_SEED],
        bump,
    )]
    /// CHECK:
    pub pda: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = flap_mint,
        token::authority = pda,
    )]
    pub flap_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimPrize<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.flap_vault.to_account_info().clone(),
            to: self.winner_flap_account.to_account_info().clone(),
            authority: self.pda.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn claime_prize_handler(
    ctx: Context<ClaimPrize>,
) -> Result<()> {
    let amount: u64 = ctx.accounts.raffle_info.prize_amount;
    let bump: &[u8; 1] = &[ctx.bumps.pda];
    let seeds: &[&[u8]] = &[AUTHORITY_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    token::transfer(ctx.accounts.transfer_context().with_signer(signer_seeds), amount)?;
    ctx.accounts.raffle_info.claimed = true;
    Ok(())
}