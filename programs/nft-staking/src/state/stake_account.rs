use anchor_lang::prelude::*;

#[account]

// per nft
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey, // mint of the nft that was staked
    pub last_update: i64,
    pub bump: u8,
}

impl Space for StakeAccount {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
