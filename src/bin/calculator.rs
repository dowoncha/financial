use std::io::{self, Read, Write};

fn try_main() -> Result<(), Box<std::error::Error>> {
    loop {
        println!("Welcome to Financial Calculator");
        println!("What would you like to calculate?
            1. Bonds
            2. Cash Flow
            3. Interest
        ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
    }

    Ok(())
}

fn main() {
    try_main().unwrap();
}