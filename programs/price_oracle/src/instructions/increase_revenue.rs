use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::state::{Asset, AssetAccount};

#[derive(Accounts)]
pub struct IncreaseRevenue<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    mut,
    seeds = [asset_mint.key().as_ref()],
    bump
    )]
    pub asset: Account<'info, Asset>,
    pub asset_mint: Account<'info, Mint>
}

pub fn increase_revenue(ctx: Context<IncreaseRevenue>, amount: u64) -> Result<()> {
    ctx.accounts.asset.cumulative_revenue += amount;
    ctx.accounts.asset.emit_update();
    Ok(())
}