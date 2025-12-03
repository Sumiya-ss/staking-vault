
use anchor_lang::prelude::*;

#[account]
pub struct StakeAccount {
    pub user: Pubkey,           // 32
    pub amount_staked: u64,     // 8
    pub stake_timestamp: i64,   // 8 (must be i64)
    pub rewards_claimed: u64,   // 8
    pub last_claim_timestamp: i64, // 8 (must be i64)
    pub bump: u8,               // 1
}


impl StakeAccount {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 8 + 1;

    pub fn calculate_rewards(&self, current_time: i64, reward_rate: u64) -> u64 {
        if self.amount_staked == 0 {
            return 0;
        }

        if current_time <= self.last_claim_timestamp {
            return 0;
        }

        let time_staked = (current_time - self.last_claim_timestamp) as u64;
        let seconds_per_year: u64 = 365 * 24 * 60 * 60; // 31,536,000

        let rewards = (self.amount_staked as u128)
            .checked_mul(time_staked as u128)
            .unwrap()
            .checked_mul(reward_rate as u128)
            .unwrap()
            .checked_div(seconds_per_year as u128)
            .unwrap()
            .checked_div(10000) // Divide by 10000 for basis points
            .unwrap();

        rewards as u64
    }
}