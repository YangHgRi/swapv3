#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    SOL,
    USDC,
}

impl TokenType {
    pub fn decimals(&self) -> u8 {
        match self {
            TokenType::SOL => 9,
            TokenType::USDC => 6,
        }
    }

    pub fn format(&self, amount: u64) -> f64 {
        let decimals = self.decimals() as i32;
        amount as f64 / 10f64.powi(decimals)
    }
}

#[derive(Debug)]
pub struct Pool {
    pub token_0: TokenType,
    pub token_1: TokenType,
    pub fee: u32,
    pub tick_spacing: u16,
    pub total_fee0: u128,
    pub total_fee1: u128,
    pub activity_liquidity: u128,
    pub sqrt_price_x64: u128,
    pub current_tick: i32,
    pub balance0: u128,
    pub balance1: u128,
}

pub struct Tick {
    pub tick: i32,
    pub liquidity: u128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_0: u128,
    pub fee_growth_outside_1: u128,
}

#[derive(Debug)]
pub struct Position {
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u128,
    pub collected_fee0: u128,
    pub collected_fee1: u128,
}
