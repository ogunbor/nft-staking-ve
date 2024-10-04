use anchor_lang::prelude::*;

declare_id!("6P8GxDkLR92adjr8RsWSDWdvvxZTciFYPCszgfen7nVM");

mod instructions;
mod state;

pub use instructions::*;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
