extern crate financial;

use financial::MACalculator;

use std::io::{self, Read, Write};

fn try_main() -> Result<(), Box<std::error::Error>>{
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <num periods>", args[0]);
        return Ok(());
    }

    let periods = args[1]
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    let mut mac = MACalculator::new(periods);

    loop {
        print!("Price: ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).ok().expect("Couldn't read input");

        match buffer.trim().parse::<f64>() {
            Ok(price) => mac.add_price_quote(price),
            Err(_) => {
                break;
            }
        }
    }

    let ma = mac.calc_ma();

    for (index, mi) in ma.iter().enumerate() {
        println!("Average value {} = {}", index, mi);
    }

    let ema = mac.calc_ema();
    
    for (index, emi) in ema.iter().enumerate() {
        println!("Exponential average value {} = {}", index, emi);
    }

    Ok(())
}

fn main() {
    try_main().unwrap();
}