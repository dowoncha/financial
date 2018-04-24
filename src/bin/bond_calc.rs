extern crate financial;

use financial::{BondCalculator};

fn try_main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    println!("{}", args.len());

    if args.len() != 5 {
        println!("usage: {} <issuer> <principal> <coupon> <num periods>", args[0]);
        return Ok(());
    }

    let issuer = &args[1];
    let principal = args[2].parse::<f64>().map_err(|e| e.to_string())?;
    let coupon = args[3].parse::<f64>().map_err(|e| e.to_string())?;
    let num_periods = args[4].parse::<usize>().map_err(|e| e.to_string())?;

    let bc = BondCalculator::new(issuer, num_periods, principal, coupon);

    println!("Reading information for bond issued by: {}", issuer);
    println!("Internal rate of return is {}", bc.interest_rate());

    Ok(())
}

fn main() {
    try_main().unwrap();
}