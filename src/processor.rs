use crate::{
    error::SwapV3Error,
    instruction::SwapV3Instruction,
    state::{Pool, Position, Tick},
};
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// The main processor for the SwapV3 program.
pub struct Processor;

impl Processor {
    /// Processes an instruction from the Solana runtime.
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // Deserialize the instruction data to determine which action to take.
        let instruction = SwapV3Instruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Route to the appropriate handler based on the instruction.
        match instruction {
            SwapV3Instruction::InitializePool { initial_price } => {
                msg!("Instruction: InitializePool");
                Self::process_initialize_pool(accounts, initial_price, program_id)
            }
            SwapV3Instruction::AddLiquidity {
                liquidity_amount,
                tick_lower,
                tick_upper,
            } => {
                msg!("Instruction: AddLiquidity");
                Self::process_add_liquidity(
                    accounts,
                    liquidity_amount,
                    tick_lower,
                    tick_upper,
                    program_id,
                )
            }
            SwapV3Instruction::Swap {
                amount_in,
                min_amount_out,
            } => {
                msg!("Instruction: Swap");
                Self::process_swap(accounts, amount_in, min_amount_out, program_id)
            }
        }
    }

    /// Processes the InitializePool instruction.
    fn process_initialize_pool(
        accounts: &[AccountInfo],
        initial_price: u128,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let token_0_mint = next_account_info(account_info_iter)?;
        let token_1_mint = next_account_info(account_info_iter)?;

        // Ensure the pool account is owned by the program.
        if pool_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut pool_data = pool_account.try_borrow_mut_data()?;
        let mut pool = Pool::try_from_slice(&pool_data)?;

        pool.token_0_mint = *token_0_mint.key;
        pool.token_1_mint = *token_1_mint.key;
        pool.sqrt_price_x64 = initial_price;
        // Default fee and tick_spacing, can be made configurable
        pool.fee = 500; // 0.05%
        pool.tick_spacing = 10;

        pool.serialize(&mut *pool_data)?;

        Ok(())
    }

    /// Processes the AddLiquidity instruction.
    fn process_add_liquidity(
        accounts: &[AccountInfo],
        liquidity_amount: u128,
        tick_lower_idx: i32,
        tick_upper_idx: i32,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let position_account = next_account_info(account_info_iter)?;
        let tick_lower_account = next_account_info(account_info_iter)?;
        let tick_upper_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = Pool::try_from_slice(&pool_account.try_borrow_data()?)?;
        let mut position = Position::try_from_slice(&position_account.try_borrow_data()?)?;
        let mut tick_lower = Tick::try_from_slice(&tick_lower_account.try_borrow_data()?)?;
        let mut tick_upper = Tick::try_from_slice(&tick_upper_account.try_borrow_data()?)?;

        position.liquidity += liquidity_amount;
        tick_lower.liquidity += liquidity_amount as i128;
        tick_upper.liquidity -= liquidity_amount as i128;

        if pool.current_tick >= tick_lower_idx && pool.current_tick < tick_upper_idx {
            pool.activity_liquidity += liquidity_amount;
        }

        pool.serialize(&mut *pool_account.try_borrow_mut_data()?)?;
        position.serialize(&mut *position_account.try_borrow_mut_data()?)?;
        tick_lower.serialize(&mut *tick_lower_account.try_borrow_mut_data()?)?;
        tick_upper.serialize(&mut *tick_upper_account.try_borrow_mut_data()?)?;

        Ok(())
    }

    /// Processes the Swap instruction.
    fn process_swap(
        accounts: &[AccountInfo],
        amount_in: u64,
        min_amount_out: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = Pool::try_from_slice(&pool_account.try_borrow_data()?)?;

        // Simplified swap logic: assume token0 is being swapped for token1
        // This does not account for concentrated liquidity and is for demonstration only.
        let amount_in_u128 = amount_in as u128;
        let price = pool.sqrt_price_x64; // This is sqrt_price_x64

        // Simplified amount_out calculation
        let amount_out = crate::utils::mul_div(amount_in_u128, price, 1 << 64)?;

        if amount_out < min_amount_out as u128 {
            return Err(SwapV3Error::InvalidPriceLimit.into());
        }

        // Simplified price update
        pool.sqrt_price_x64 = price + (amount_in_u128 / 1000); // Arbitrary price change

        pool.serialize(&mut *pool_account.try_borrow_mut_data()?)?;

        Ok(())
    }
}
