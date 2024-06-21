use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct UpdateAsset<'info> {
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

pub fn update_asset(ctx: Context<UpdateAsset>, price: u64, appreciation_rate: u32, rent: u8) -> Result<()> {
    let asset = &mut ctx.accounts.asset;
    asset.price = price;
    asset.appreciation_rate = appreciation_rate;
    asset.rent = rent;

    let timestamp = Clock::get().unwrap().unix_timestamp;
    emit!(UpdateAssetEvent{
        id: asset.id,
        price,
        appreciation_rate,
        rent,
        timestamp
    });
    Ok(())
}