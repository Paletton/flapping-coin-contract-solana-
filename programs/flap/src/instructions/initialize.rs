use anchor_lang::prelude::*;

use crate::{AppStats, APP_STATS_SEED};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    pub admin: UncheckedAccount<'info>,

    /// CHECK:
    pub flap_mint: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        space = size_of::<AppStats>() + 8,
        seeds = [APP_STATS_SEED],
        bump,
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    app_stats.admin = ctx.accounts.admin.key();
    app_stats.flap_mint = ctx.accounts.flap_mint.key();
    app_stats.total_amount = 0;
    app_stats.player_count = 0;
    app_stats.is_initialized = true;
    Ok(())
}
