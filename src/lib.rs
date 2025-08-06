// Declare the modules that make up the program.
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

// Define the program's entrypoint.
// The Solana runtime calls this function for every instruction sent to the program.
entrypoint!(process_instruction);

/// The main instruction processing function.
/// It routes the instruction to the processor.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Call the processor to handle the instruction.
    processor::Processor::process(program_id, accounts, instruction_data)
}
