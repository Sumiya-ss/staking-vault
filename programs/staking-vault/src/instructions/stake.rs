use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::StakeAccount;
use crate::errors::StakingError;
use crate::constants::{MIN_STAKE_AMOUNT, MAX_STAKE_AMOUNT, REWARD_RATE_BPS};

/// Stake tokens
pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    require!(amount >= MIN_STAKE_AMOUNT, StakingError::AmountTooSmall);
    require!(amount <= MAX_STAKE_AMOUNT, StakingError::AmountTooLarge);
    
    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;
    
    // If already staking, claim pending rewards first
    if stake_account.amount_staked > 0 {
        let pending_rewards = stake_account.calculate_rewards(
            clock.unix_timestamp,
            REWARD_RATE_BPS
        );
        
        if pending_rewards > 0 {
            stake_account.rewards_claimed = stake_account
                .rewards_claimed
                .checked_add(pending_rewards)
                .ok_or(StakingError::MathOverflow)?;
        }
    }
    
    // Transfer tokens from user to vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    
    // Update stake account
    stake_account.amount_staked = stake_account
        .amount_staked
        .checked_add(amount)
        .ok_or(StakingError::MathOverflow)?;
    
    stake_account.last_claim_timestamp = clock.unix_timestamp;
    
    msg!("Staked {} tokens. Total staked: {}", amount, stake_account.amount_staked);
    
    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref()],
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
   
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}