const x64: u128 = 1 << 64;

pub fn sqrt_price(price: u128) -> f64 {
    (price / x64) as f64
}
