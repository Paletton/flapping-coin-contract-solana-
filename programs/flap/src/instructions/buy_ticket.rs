use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{error::ErrorCode, Raffle, RaffleInfo, AUTHORITY_SEED, RAFFLE_SEED, STABLE_VAULT_SEED};

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

    pub stable_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::mint = stable_mint,
        token::authority = signer,
    )]
    pub stable_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [AUTHORITY_SEED],
        bump,
    )]
    /// CHECK:
    pub pda: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        token::mint = stable_mint,
        token::authority = pda,
        seeds = [STABLE_VAULT_SEED, stable_mint.key().as_ref()],
        bump
    )]
    pub stable_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyTicket<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.stable_account.to_account_info().clone(),
            to: self.stable_vault.to_account_info().clone(),
            authority: self.signer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}


pub fn buy_ticket_handler(ctx: Context<BuyTicket>, ticket_demand: u16) -> Result<()> {
    let amount: u64 = ctx.accounts.raffle_info.ticket_price * ticket_demand as u64;
    token::transfer(ctx.accounts.transfer_context(), amount)?;
    let timestamp: i64 = Clock::get()?.unix_timestamp;
    let mut raffle: std::cell::RefMut<'_, Raffle> = ctx.accounts.raffle.load_mut()?;
    let raffle_info: &mut Box<Account<'_, RaffleInfo>> = &mut ctx.accounts.raffle_info;
    raffle_info.total_collect += amount;
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
