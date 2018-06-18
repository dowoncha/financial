extern crate financial;
use financial::{BarFeed, Bars};
use financial::strategy::{BacktestingStrategy};

use std::error::Error;

struct MyStrategy {
    base: BacktestingStrategy,
    instrument: &'static str
}

impl MyStrategy {
    pub fn new(bar_feed: impl BarFeed, instrument: &str) -> Self {
        Self {
            base: BacktestingStrategy::new(bar_feed),
            instrument
        }
    } 
}

impl Strategy for MyStrategy {
    fn base(&self) -> &BaseStrategy {
        &self.base
    }

    fn on_bars(&self, bars: Bars) {
        let bar = bars.get(self.instrument)
        self.info(bar.close())
    }
}

fn main() -> Result<(), Box<Error>> {
    let feed = CsvFeed;

    let strategy: MYStrategy::new(feed);
    strategy.run();

    Ok(())
}