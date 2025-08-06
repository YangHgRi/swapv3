use borsh::{BorshDeserialize, BorshSerialize};

/// Defines the instructions that the SwapV3 program can process.
/// Each variant corresponds to a specific action that can be performed.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum SwapV3Instruction {
    /// Initializes a new liquidity pool.
    ///
    /// Accounts expected:
    /// 0. `[writable]` The pool account to initialize.
    /// 1. `[]` The mint for token_0.
    /// 2. `[]` The mint for token_1.
    /// 3. `[]` The rent sysvar.
    InitializePool {
        /// The initial price of the pool.
        initial_price: u128,
    },

    /// Adds liquidity to a specified range in the pool.
    ///
    /// Accounts expected:
    /// 0. `[writable]` The pool account.
    /// 1. `[writable]` The position account for the user.
    /// 2. `[writable]` The tick account for the lower bound.
    /// 3. `[writable]` The tick account for the upper bound.
    /// 4. `[signer]` The owner of the position.
    AddLiquidity {
        /// The amount of liquidity to add.
        liquidity_amount: u128,
        /// The lower tick boundary for the liquidity.
        tick_lower: i32,
        /// The upper tick boundary for the liquidity.
        tick_upper: i32,
    },

    /// Swaps one token for another in the pool.
    ///
    /// Accounts expected:
    /// 0. `[writable]` The pool account.
    /// 1. `[signer]` The user initiating the swap.
    /// 2. `[writable]` The user's source token account.
    /// 3. `[writable]` The user's destination token account.
    /// 4. `[writable]` The pool's source token vault.
    /// 5. `[writable]` The pool's destination token vault.
    /// 6. `[]` The SPL Token program.
    Swap {
        /// The amount of the input token to swap.
        amount_in: u64,
        /// The minimum amount of the output token the user is willing to accept.
        min_amount_out: u64,
    },
}
