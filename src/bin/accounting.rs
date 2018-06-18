extern crate chrono;

use chrono::prelude::*;

pub type AccountId = i32;

enum Currency {
    USD
}

struct Cash {
    data: u64,
    currency: Currency
}

impl Cash {
    pub fn value(&self) -> f64 {
        data as f64 / 100
    }
}

struct JournalEntry {
    date: DateTime<Local>,
    description: String,
    debits: (AccountId, Cash),
    credits: (AccountId, Cash) 
}

impl JournalEntry {
    pub fn total_debits(&self) -> Cash {
        let mut total = 0;
        
        for (_, cash) in self.debits {
            total += cash
        }

        total
    }   

    pub fn total_credits(&self) -> Cash {
        let mut total = 0;

        for (_, cash) in self.credits {
            total += cash
        } 

        total
    }
}

mod reports {

pub struct BalanceSheet {

}

pub struct IncomeStatement {

}

}

static MAIN_MENU_PROMPT: &'static str = "
    1. Add Journal Entry
    2. Add Account
    3. Quit
";

fn main() -> Result<(), Box<std::error::Error>> {


    loop {
        let mut input_buffer = String::new();
        print!("account> ");
        io::stdin().read_to_string(&mut buffer);

    }    
}