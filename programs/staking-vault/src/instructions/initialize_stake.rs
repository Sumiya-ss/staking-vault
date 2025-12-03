
use anchor_lang::prelude::*;
use crate::state::StakeAccount;

/// Initializing a stake account for a user
pub fn initialize_stake(ctx: Context<InitializeStake>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;
    
    stake_account.user = ctx.accounts.user.key();
    stake_account.amount_staked = 0;
    stake_account.stake_timestamp = clock.unix_timestamp;
    stake_account.rewards_claimed = 0;
    stake_account.last_claim_timestamp = clock.unix_timestamp;
    stake_account.bump = ctx.bumps.stake_account;
    
    msg!("Stake account initialized for user: {}", stake_account.user);
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeStake<'info> {
    #[account(
        init,
        payer = user,
        space = StakeAccount::LEN,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}