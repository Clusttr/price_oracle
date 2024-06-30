use anchor_lang::prelude::*;
use anchor_spl::token::{Mint};
use crate::state::*;

#[derive(Accounts)]
pub struct InitAsset<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    init,
    seeds = [asset_mint.key().as_ref()],
    bump,
    payer = signer,
    space = Asset::SIZE
    )]
    pub asset: Account<'info, Asset>,
    pub asset_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>
}

pub fn init_asset(ctx: Context<InitAsset>, value: u64, appreciation_rate: u16, rent: u32) -> Result<()> {
    ctx.accounts.asset.set_inner(
        Asset::new(ctx.accounts.asset_mint.key(), value, appreciation_rate, rent)
    );

    let block = Clock::get()?.slot;
    let init_asset_event = InitAssetEvent{
        id: ctx.accounts.asset.key(),
        value,
        appreciation_rate,
        rent,
        cumulative_revenue: 0,
        cumulative_maintenance_cost: 0,
        block
    };
    let init_asset_event_str = init_asset_event.stringify();

    msg!(&init_asset_event_str);
    emit!(init_asset_event);
    Ok(())
}