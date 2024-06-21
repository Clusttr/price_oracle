mod state;
mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("2QQpxGtYLFqKZp5SSejPBMPfWUGq1K3GKN8gEtJUgf6q");

#[program]
pub mod price_oracle {
    use super::*;

    pub fn init_asset(ctx: Context<InitAsset>,
                      price: u64,
                      appreciation_rate: u32,
                      rent: u8) -> Result<()> {
        instructions::init_asset(ctx, price, appreciation_rate, rent)
    }

    pub fn update_asset(ctx: Context<UpdateAsset>,
                        price: u64,
                        appreciation_rate: u32,
                        rent: u8) -> Result<()> {
        instructions::update_asset(ctx, price, appreciation_rate, rent)
    }
}
