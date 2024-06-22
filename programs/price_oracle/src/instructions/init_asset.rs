use anchor_lang::prelude::*;
use anchor_spl::token::{Mint};
use crate::state::{Asset, InitAssetEvent};

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

pub fn init_asset(ctx: Context<InitAsset>, price: u64, appreciation_rate: u32, rent: u8) -> Result<()> {
    ctx.accounts.asset.set_inner(
        Asset::new(ctx.accounts.asset_mint.key(), price, appreciation_rate, rent)
    );

    let timestamp = Clock::get().unwrap().unix_timestamp;
    emit!(InitAssetEvent{
        id: ctx.accounts.asset.key(),
        price,
        appreciation_rate,
        rent,
        timestamp
    });
    Ok(())
}