use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Represents a liquidity pool in the SwapV3 program.
/// This struct holds all the global state for a single pool.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Pool {
    /// The mint address of the first token (token_0).
    pub token_0_mint: Pubkey,
    /// The mint address of the second token (token_1).
    pub token_1_mint: Pubkey,
    /// The fee tier for this pool, in basis points (e.g., 500 for 0.50%).
    pub fee: u32,
    /// The spacing between usable ticks.
    pub tick_spacing: u16,
    /// The total protocol fees collected in token_0.
    pub total_fee0: u128,
    /// The total protocol fees collected in token_1.
    pub total_fee1: u128,
    /// The total active liquidity in the pool.
    pub activity_liquidity: u128,
    /// The current price of the pool, represented as a Q64.64 fixed-point number.
    pub sqrt_price_x64: u128,
    /// The current tick index of the pool.
    pub current_tick: i32,
}

impl Pool {
    // Define the size of the Pool struct for account allocation.
    // Pubkey (32) * 2 + u32 (4) + u16 (2) + u128 (16) * 4 + i32 (4) = 64 + 4 + 2 + 64 + 4 = 138
    // Add padding for future expansion.
    pub const LEN: usize = 32 + 32 + 4 + 2 + 16 + 16 + 16 + 16 + 4 + 128;
}

/// Represents a single tick in the price range.
/// Ticks are used to track liquidity changes at specific price points.
#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Copy, Clone)]
pub struct Tick {
    /// The tick index.
    pub tick: i32,
    /// The net change in liquidity when this tick is crossed.
    pub liquidity: i128,
    /// The total fee growth for token_0 outside of this tick.
    pub fee_growth_outside_0: u128,
    /// The total fee growth for token_1 outside of this tick.
    pub fee_growth_outside_1: u128,
}

impl Tick {
    // i32 (4) + i128 (16) + u128 (16) * 2 = 4 + 16 + 32 = 52
    pub const LEN: usize = 4 + 16 + 16 + 16;
}

/// Represents a user's liquidity position in a specific range.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Position {
    /// The owner of this position.
    pub owner: Pubkey,
    /// The lower tick boundary of the position.
    pub tick_lower: i32,
    /// The upper tick boundary of the position.
    pub tick_upper: i32,
    /// The amount of liquidity provided by this position.
    pub liquidity: u128,
    /// The fees in token_0 collected since the last withdrawal.
    pub collected_fee0: u128,
    /// The fees in token_1 collected since the last withdrawal.
    pub collected_fee1: u128,
}

impl Position {
    // Pubkey (32) + i32 (4) * 2 + u128 (16) * 3 = 32 + 8 + 48 = 88
    pub const LEN: usize = 32 + 4 + 4 + 16 + 16 + 16;
}
