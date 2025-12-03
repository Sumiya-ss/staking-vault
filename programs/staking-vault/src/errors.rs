use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Amount must be greater than minimum stake")]
    AmountTooSmall,
    
    #[msg("Amount exceeds maximum stake limit")]
    AmountTooLarge,
    
    #[msg("No tokens staked")]
    NoTokensStaked,
    
    #[msg("Insufficient staked balance")]
    InsufficientBalance,
    
    #[msg("Unauthorized: only authority can perform this action")]
    Unauthorized,
    
    #[msg("Calculation overflow")]
    MathOverflow,
}