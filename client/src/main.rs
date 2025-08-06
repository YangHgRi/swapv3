use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;
use swapv3::instruction::SwapV3Instruction;
use swapv3::state::{Pool, Position, Tick};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    /// The RPC URL for the Solana cluster.
    #[clap(short, long, default_value = "http://127.0.0.1:8899")]
    rpc_url: String,
    /// The program ID of the SwapV3 program.
    #[clap(long)]
    program_id: String,
    /// The keypair of the user.
    #[clap(long)]
    fee_payer: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new liquidity pool.
    InitializePool {
        #[clap(long)]
        token_a_mint: String,
        #[clap(long)]
        token_b_mint: String,
        #[clap(long)]
        initial_price: u128,
    },
    /// Adds liquidity to a pool.
    AddLiquidity {
        #[clap(long)]
        pool_address: String,
        #[clap(long)]
        amount: u128,
        #[clap(long)]
        tick_lower: i32,
        #[clap(long)]
        tick_upper: i32,
    },
    /// Swaps tokens in a pool.
    Swap {
        #[clap(long)]
        pool_address: String,
        #[clap(long)]
        amount_in: u64,
        #[clap(long)]
        min_amount_out: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = RpcClient::new(cli.rpc_url);
    let program_id = Pubkey::from_str(&cli.program_id)?;
    let fee_payer = Keypair::from_base58_string(&cli.fee_payer);

    match cli.command {
        Commands::InitializePool {
            token_a_mint,
            token_b_mint,
            initial_price,
        } => {
            println!("Initializing pool...");
            let pool_account = Keypair::new();
            let token_a_mint_pubkey = Pubkey::from_str(&token_a_mint)?;
            let token_b_mint_pubkey = Pubkey::from_str(&token_b_mint)?;

            let space = Pool::LEN;
            let rent_exemption_amount = client.get_minimum_balance_for_rent_exemption(space)?;

            let create_account_instruction = system_instruction::create_account(
                &fee_payer.pubkey(),
                &pool_account.pubkey(),
                rent_exemption_amount,
                space as u64,
                &program_id,
            );

            let initialize_instruction = SwapV3Instruction::InitializePool { initial_price };
            let instruction = solana_sdk::instruction::Instruction::new_with_borsh(
                program_id,
                &initialize_instruction,
                vec![
                    solana_sdk::instruction::AccountMeta::new(pool_account.pubkey(), false),
                    solana_sdk::instruction::AccountMeta::new_readonly(token_a_mint_pubkey, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(token_b_mint_pubkey, false),
                ],
            );

            let mut transaction =
                Transaction::new_with_payer(&[create_account_instruction, instruction], Some(&fee_payer.pubkey()));
            transaction.sign(&[&fee_payer, &pool_account], client.get_latest_blockhash()?);
            client.send_and_confirm_transaction(&transaction)?;

            println!("Pool created at address: {}", pool_account.pubkey());
        }
        Commands::AddLiquidity {
            pool_address,
            amount,
            tick_lower,
            tick_upper,
        } => {
            println!("Adding liquidity...");
            let pool_pubkey = Pubkey::from_str(&pool_address)?;
            let position_account = Keypair::new();
            let tick_lower_account = Keypair::new();
            let tick_upper_account = Keypair::new();

            let create_position_instruction = system_instruction::create_account(
                &fee_payer.pubkey(),
                &position_account.pubkey(),
                client.get_minimum_balance_for_rent_exemption(Position::LEN)?,
                Position::LEN as u64,
                &program_id,
            );
            let create_tick_lower_instruction = system_instruction::create_account(
                &fee_payer.pubkey(),
                &tick_lower_account.pubkey(),
                client.get_minimum_balance_for_rent_exemption(Tick::LEN)?,
                Tick::LEN as u64,
                &program_id,
            );
            let create_tick_upper_instruction = system_instruction::create_account(
                &fee_payer.pubkey(),
                &tick_upper_account.pubkey(),
                client.get_minimum_balance_for_rent_exemption(Tick::LEN)?,
                Tick::LEN as u64,
                &program_id,
            );

            let add_liquidity_instruction = SwapV3Instruction::AddLiquidity {
                liquidity_amount: amount,
                tick_lower,
                tick_upper,
            };
            let instruction = solana_sdk::instruction::Instruction::new_with_borsh(
                program_id,
                &add_liquidity_instruction,
                vec![
                    solana_sdk::instruction::AccountMeta::new(pool_pubkey, false),
                    solana_sdk::instruction::AccountMeta::new(position_account.pubkey(), false),
                    solana_sdk::instruction::AccountMeta::new(tick_lower_account.pubkey(), false),
                    solana_sdk::instruction::AccountMeta::new(tick_upper_account.pubkey(), false),
                    solana_sdk::instruction::AccountMeta::new_readonly(fee_payer.pubkey(), true),
                ],
            );

            let mut transaction = Transaction::new_with_payer(
                &[
                    create_position_instruction,
                    create_tick_lower_instruction,
                    create_tick_upper_instruction,
                    instruction,
                ],
                Some(&fee_payer.pubkey()),
            );
            transaction.sign(
                &[
                    &fee_payer,
                    &position_account,
                    &tick_lower_account,
                    &tick_upper_account,
                ],
                client.get_latest_blockhash()?,
            );
            client.send_and_confirm_transaction(&transaction)?;

            println!("Liquidity added. Position account: {}", position_account.pubkey());
        }
        Commands::Swap {
            pool_address,
            amount_in,
            min_amount_out,
        } => {
            println!("Swapping tokens...");
            let pool_pubkey = Pubkey::from_str(&pool_address)?;

            let swap_instruction = SwapV3Instruction::Swap {
                amount_in,
                min_amount_out,
            };
            let instruction = solana_sdk::instruction::Instruction::new_with_borsh(
                program_id,
                &swap_instruction,
                vec![
                    solana_sdk::instruction::AccountMeta::new(pool_pubkey, false),
                    solana_sdk::instruction::AccountMeta::new_readonly(fee_payer.pubkey(), true),
                ],
            );

            let mut transaction =
                Transaction::new_with_payer(&[instruction], Some(&fee_payer.pubkey()));
            transaction.sign(&[&fee_payer], client.get_latest_blockhash()?);
            client.send_and_confirm_transaction(&transaction)?;

            println!("Swap successful.");
        }
    }

    Ok(())
}