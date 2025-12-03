use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::REWARD_RATE_BPS;
use crate::errors::StakingError;
use crate::state::StakeAccount;

/// Claim rewards without unstaking principal
pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;

    let pending_rewards =
        stake_account.calculate_rewards(clock.unix_timestamp, REWARD_RATE_BPS);
    require!(pending_rewards > 0, StakingError::NoTokensStaked);

    // Transfer only the rewards from vault to user
    let seeds = &[
        b"vault".as_ref(),
        &[ctx.bumps.vault_authority],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, pending_rewards)?;

    // Update accounting
    stake_account.rewards_claimed = stake_account
        .rewards_claimed
        .checked_add(pending_rewards)
        .ok_or(StakingError::MathOverflow)?;
    stake_account.last_claim_timestamp = clock.unix_timestamp;

    msg!(
        "Claimed {} rewards for user {}",
        pending_rewards,
        stake_account.user
    );

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref()],
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    /// User's token account (destination)
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    /// Vault token account (source)
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// Vault authority PDA (can sign for vault)
    /// CHECK: PDA signer
    #[account(
        seeds = [b"vault"],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}