use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{AppStats, Player, APP_STATS_SEED, AUTHORITY_SEED, PLAYER_SEED, error::ErrorCode};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,


    #[account(
        mut,
        seeds = [PLAYER_SEED, owner.key().as_ref()],
        bump,
        has_one = owner,
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
        mut,
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
}

impl<'info> Withdraw<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.flap_vault.to_account_info().clone(),
            to: self.player_flap_account.to_account_info().clone(),
            authority: self.pda.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn withdraw_handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    if amount > ctx.accounts.player.balance {
        return err!(ErrorCode::InsufficientBalance);
    }
    let bump: &[u8; 1] = &[ctx.bumps.pda];
    let seeds: &[&[u8]] = &[AUTHORITY_SEED, bump];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    token::transfer(ctx.accounts.transfer_context().with_signer(signer_seeds), amount)?;
    let player: &mut Box<Account<'_, Player>> = &mut ctx.accounts.player;
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    player.balance -= amount;
    app_stats.total_amount -= amount;
    Ok(())
}