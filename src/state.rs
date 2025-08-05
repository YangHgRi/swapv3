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
