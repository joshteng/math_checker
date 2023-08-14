mod math;
use math::Math;

fn calculate_fee(total_fee: u128) -> Result<u64, String> {
    let u128_fee = Math::checked_div(Math::checked_mul(total_fee, 10)?, 10000)?;
    let fee: u64 = Math::checked_as_u64(u128_fee)?;

    Ok(fee)
}

fn main() {
    match calculate_fee(1) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    match Math::checked_sub(100, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
