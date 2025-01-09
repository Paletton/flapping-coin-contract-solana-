use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{error::ErrorCode, AppStats, Player, APP_STATS_SEED, AUTHORITY_SEED, PLAYER_SEED};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        space = size_of::<Player>() + 8,
        seeds = [PLAYER_SEED, owner.key().as_ref()],
        bump,
    )]
    pub player: Box<Account<'info, Player>>,

    pub flap_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::mint = flap_mint,
        token::authority = player,
    )]
    pub player_flap_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [AUTHORITY_SEED],
        bump,
    )]
    /// CHECK:
    pub pda: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        token::mint = flap_mint,
        token::authority = pda,
    )]
    pub flap_vault: Box<Account<'info, TokenAccount>>,

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

impl<'info> Deposit<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.player_flap_account.to_account_info().clone(),
            to: self.flap_vault.to_account_info().clone(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn deposit_handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    token::transfer(ctx.accounts.transfer_context(), amount)?;
    let player: &mut Box<Account<'_, Player>> = &mut ctx.accounts.player;
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    app_stats.total_amount += amount;
    if !player.initialized {
        player.balance = amount;
        player.owner = ctx.accounts.owner.key();
        player.initialized = true;
        app_stats.player_count += 1;
    } else {
        if player.owner != ctx.accounts.owner.key() {
            return err!(ErrorCode::InvalidPlayer);
        }
        player.balance += amount;
    }
    Ok(())
}
