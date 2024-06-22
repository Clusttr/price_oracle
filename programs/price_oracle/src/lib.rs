mod state;
mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("2QQpxGtYLFqKZp5SSejPBMPfWUGq1K3GKN8gEtJUgf6q");

#[program]
pub mod price_oracle {
    use super::*;

    pub fn init_asset(ctx: Context<InitAsset>,
                      value: u64,
                      appreciation_rate: u16,
                      rent: u32) -> Result<()> {
        instructions::init_asset(ctx, value, appreciation_rate, rent)
    }

    pub fn update_asset(ctx: Context<UpdateAsset>,
                        value: u64,
                        appreciation_rate: u16,
                        rent: u32) -> Result<()> {
        instructions::update_asset(ctx, value, appreciation_rate, rent)
    }

    pub fn increase_revenue(ctx: Context<IncreaseRevenue>, amount: u64) -> Result<()> {
        instructions::increase_revenue(ctx, amount)
    }

    pub fn increase_cost(ctx: Context<IncreaseCost>, amount: u64) -> Result<()> {
        instructions::increase_cost(ctx, amount)
    }
}
