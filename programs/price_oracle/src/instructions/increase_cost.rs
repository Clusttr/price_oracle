use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::state::{Asset, AssetAccount};

#[derive(Accounts)]
pub struct IncreaseCost<'info> {
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

pub fn increase_cost(ctx: Context<IncreaseCost>, amount: u64) -> Result<()> {
    ctx.accounts.asset.cumulative_maintenance_cost += amount;
    ctx.accounts.asset.emit_update();
    Ok(())
}