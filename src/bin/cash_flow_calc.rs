extern crate financial;

use financial::{CashFlowCalculator};

use std::io::{self, Read};

fn try_main() -> Result<(), Box<std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage {} <interest rate>", args[0]);
        return Ok(())
    }

    let rate = args[1].parse::<f64>()?;

    let mut cfc = CashFlowCalculator::new(rate);

    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        let mut words = buffer.split_whitespace();
        let period = words
                        .next()
                        .expect("No period input")
                        .parse::<i32>()?;
        if period == -1 {
            break;
        }

        let value = words
                        .next()
                        .expect("No principle input")
                        .parse::<f64>()?;
        cfc.add_payment(value, period);
    }

    let result = cfc.present_value();
    println!("The present value is: {}", result);

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        println!("Error: {:?}", e);
    }
}

