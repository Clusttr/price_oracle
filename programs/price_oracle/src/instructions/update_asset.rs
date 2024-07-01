use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateAsset<'info> {
    #[account(
        mut,
        address = Pubkey::from_str(constants::ADMIN).unwrap()
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [asset_mint.key().as_ref()],
        bump
    )]
    pub asset: Account<'info, Asset>,
    pub asset_mint: Account<'info, Mint>
}

pub fn update_asset(ctx: Context<UpdateAsset>, value: u64, appreciation_rate: u16, rent: u32) -> Result<()> {
    let asset = &mut ctx.accounts.asset;
    asset.value = value;
    asset.appreciation_rate = appreciation_rate;
    asset.rent = rent;

    ctx.accounts.asset.emit_update();
    Ok(())
}