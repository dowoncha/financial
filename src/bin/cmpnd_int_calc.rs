extern crate financial;

use financial::{CompoundIntRateCalculator};

fn try_main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        println!("usage: {} <interest rate> <present value> <num periods>", args[0]);
        return Ok(());
    }

    let rate = args[1].parse::<f64>()?;
    let principle = args[2].parse::<f64>()?;
    let num_periods = args[3].parse::<i32>()?;

    let mut ci_calc = CompoundIntRateCalculator::new();
    ci_calc.set_rate(rate);
    let discrete = ci_calc.multiple_period(principle, num_periods);
    let continuous = ci_calc.continuous_compound(principle, num_periods);
    
    println!("Future value for multiple period compounding is: {}", discrete);
    println!("Future value for continuous compounding is: {}", continuous);

    Ok(())
}

fn main() {
    try_main().unwrap();
}