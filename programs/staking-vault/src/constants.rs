
use anchor_lang::prelude::*;

pub const AUTHORITY: Pubkey = pubkey!("JDuJ2DDZ6MxuSedmU7v7tRJDbaf1kAvfwpGNocxAhp2d");

pub const REWARD_RATE_BPS: u64 = 1000;
pub const MIN_STAKE_AMOUNT: u64 = 1_000_000;
pub const MAX_STAKE_AMOUNT: u64 = 1_000_000_000_000;
