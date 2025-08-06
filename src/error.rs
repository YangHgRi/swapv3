use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Defines the custom errors that the SwapV3 program can return.
#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum SwapV3Error {
    /// Indicates that an invalid instruction was provided.
    #[error("Invalid Instruction")]
    InvalidInstruction,

    /// Indicates that a calculation resulted in an overflow.
    #[error("Calculation overflow")]
    Overflow,

    /// Indicates that an invalid tick index was provided.
    #[error("Invalid tick index")]
    InvalidTick,

    /// Indicates that an invalid price limit was provided.
    #[error("Invalid price limit")]
    InvalidPriceLimit,
}

/// Allows converting a `SwapV3Error` into a `ProgramError`,
/// which is the standard error type in Solana programs.
impl From<SwapV3Error> for ProgramError {
    fn from(e: SwapV3Error) -> Self {
        // Custom program errors in Solana are represented as u32 values.
        // We can assign a unique code to each of our custom errors.
        ProgramError::Custom(e as u32)
    }
}
