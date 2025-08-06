use crate::error::SwapV3Error;
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use solana_program::program_error::ProgramError;

/// Calculates the square root of the price for a given tick index.
/// The price is represented as a Q64.64 fixed-point number.
///
/// Formula: sqrt(p) = 1.0001^(tick / 2) * 2^64
///
/// # Arguments
/// * `tick` - The tick index.
///
/// # Returns
/// The square root of the price as a u128.
pub fn get_sqrt_ratio_at_tick(tick: i32) -> Result<u128, ProgramError> {
    let tick_abs = tick.abs();
    if tick_abs > 443636 {
        // A reasonable limit to prevent overflow
        return Err(SwapV3Error::InvalidTick.into());
    }

    let mut ratio: u128 = if (tick_abs & 0x1) != 0 {
        0xfffcb933bd6a8861u128
    } else {
        0x10000000000000000u128
    };
    if (tick_abs & 0x2) != 0 {
        ratio = (ratio * 0xfff97272373d4131u128) >> 128;
    }
    if (tick_abs & 0x4) != 0 {
        ratio = (ratio * 0xfff2e50f5f656932u128) >> 128;
    }
    if (tick_abs & 0x8) != 0 {
        ratio = (ratio * 0xffe5caca7e10e56fu128) >> 128;
    }
    if (tick_abs & 0x10) != 0 {
        ratio = (ratio * 0xffcb9843d60f6159u128) >> 128;
    }
    if (tick_abs & 0x20) != 0 {
        ratio = (ratio * 0xff973b41fa98c08eu128) >> 128;
    }
    if (tick_abs & 0x40) != 0 {
        ratio = (ratio * 0xff2ea16466c96a38u128) >> 128;
    }
    if (tick_abs & 0x80) != 0 {
        ratio = (ratio * 0xfe5c563584e73a7bu128) >> 128;
    }
    if (tick_abs & 0x100) != 0 {
        ratio = (ratio * 0xfcbe86c7900a88a8u128) >> 128;
    }
    if (tick_abs & 0x200) != 0 {
        ratio = (ratio * 0xf987a7253ac41317u128) >> 128;
    }
    if (tick_abs & 0x400) != 0 {
        ratio = (ratio * 0xf30ee3e2198a2598u128) >> 128;
    }
    if (tick_abs & 0x800) != 0 {
        ratio = (ratio * 0xe69594ea4a695914u128) >> 128;
    }
    if (tick_abs & 0x1000) != 0 {
        ratio = (ratio * 0xcf0569513744055bu128) >> 128;
    }
    if (tick_abs & 0x2000) != 0 {
        ratio = (ratio * 0x9e774231edb50266u128) >> 128;
    }
    if (tick_abs & 0x4000) != 0 {
        ratio = (ratio * 0x6675fd95340d4576u128) >> 128;
    }
    if (tick_abs & 0x8000) != 0 {
        ratio = (ratio * 0x3202b17ecdf93341u128) >> 128;
    }
    if (tick_abs & 0x10000) != 0 {
        ratio = (ratio * 0xa93819b068533333u128) >> 128;
    }
    if (tick_abs & 0x20000) != 0 {
        ratio = (ratio * 0x5555555555555555u128) >> 128;
    }

    if tick > 0 {
        // Invert the ratio for positive ticks
        let ratio_big: BigUint = (BigUint::from(1u128) << 256) / BigUint::from(ratio);
        Ok(ratio_big.to_u128().unwrap_or(0))
    } else {
        Ok(ratio)
    }
}

/// Performs a multiplication and division operation: (a * b) / c
/// This is implemented using BigUint to prevent overflow.
///
/// # Arguments
/// * `a`, `b`, `c` - The numbers to operate on.
///
/// # Returns
/// The result of the operation as a u128.
pub fn mul_div(a: u128, b: u128, c: u128) -> Result<u128, ProgramError> {
    let a_big = BigUint::from(a);
    let b_big = BigUint::from(b);
    let c_big = BigUint::from(c);

    if c_big.is_zero() {
        return Err(SwapV3Error::Overflow.into());
    }

    let result_big = (a_big * b_big) / c_big;

    // Check if the result fits within a u128
    if result_big.bits() > 128 {
        return Err(SwapV3Error::Overflow.into());
    }

    Ok(result_big.to_u128().unwrap_or(0))
}
