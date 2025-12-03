use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod constants;
pub mod instructions;

pub use instructions::*;
declare_id!("4UY1kyQkvETUn5kThQRGnqc4arNjh8vgMDx1c6o4Ew8s");


#[program]
pub mod token_staking_program {
    use super::*;

    /// Initialize a new stake account for user
    pub fn initialize_stake(ctx: Context<InitializeStake>) -> Result<()> {
        instructions::initialize_stake(ctx)
    }

    /// Stake tokens
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake(ctx, amount)
    }

    /// Unstake all tokens and claim rewards
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake(ctx)
    }

    /// Claim rewards without unstaking
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards(ctx)
    }
}