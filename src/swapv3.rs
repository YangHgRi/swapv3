use crate::state::{Pool, TokenType};
use crate::utils;

pub fn compute_other_input(
    token: TokenType,
    amount_input: u128,
    pool: &Pool,
    price_lower: u128,
    price_upper: u128,
) -> u128 {
    let current_price = pool.sqrt_price_x64;
    let pu = utils::sqrt_price(price_upper);
    let pl = utils::sqrt_price(price_lower);
    let p = utils::sqrt_price(current_price);

    let is_token0 = token.eq(&pool.token_0);

    if current_price <= price_lower || current_price > price_upper {
        return 0;
    }

    if is_token0 {
        let l = (amount_input as f64 * pu * p) / (pu - p);
        let amount_input1 = l * (p - pl);
        amount_input1 as u128
    } else {
        let l = (amount_input as f64 / (p - pl));
        let amount_other_input = l * (pu - p) / (p * pu);
        amount_other_input as u128
    }
}
