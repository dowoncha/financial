extern crate financial;

use financial::{InterestRateCalculator};


fn try_main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("usage: progName <interest rate> <value>");
    }

    let rate = args[1].parse::<f64>()?;
    let value = args[2].parse::<f64>()?;

    let calculator = {
        let mut c = InterestRateCalculator::new();
        c.set_rate(rate);
        c
    };

    let res = calculator.single_period(value);
    println!("Result is {}", res);

    Ok(())
}

fn main() {
    try_main().unwrap();
}