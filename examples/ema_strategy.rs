/**
 * Forex Strategy Backtester
 * 
 * Tests a custom strategy script using
 * a history csv file.
 * 
 * Uses FileBroker as default
 * 
 **/

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate chrono;
use chrono::{NaiveDateTime};

extern crate financial;

use financial::{Broker, 
    Strategy, StrategySettings, 
    DataPoint, Candle, MarketTrend,
    BacktestRunner};
use financial::brokers::{FileBroker, FileBrokerSettings};

use financial::indicators::{Indicator, ExponentialMovingAverage};

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

struct SimpleStrategy {
    settings: StrategySettings,
    total_pnl: f64,
    trading_enabled: Cell<bool>,
    logging_current_price: f64,
    short_ema: ExponentialMovingAverage,
    long_ema: ExponentialMovingAverage,
    is_running: bool,
    current_candle: Cell<Option<Candle>>
}

impl Default for SimpleStrategy {
    fn default() -> Self {
        Self {
            total_pnl: 0.0,
            trading_enabled: Cell::new(false),
            logging_current_price: 0.0,
            is_running: false,
            short_ema: ExponentialMovingAverage::new(10),
            long_ema: ExponentialMovingAverage::new(20),
            current_candle: Cell::new(None),
            settings: Default::default()
        }
    }
}

impl SimpleStrategy {

    const SHORT_EMA_PERIOD: usize = 7;
    const LONG_EMA_PERIOD: usize = 21;

    fn new(settings: StrategySettings) -> Self {
        let strategy = Self {
            settings,
            // TODO: Change into EMA
            short_ema: ExponentialMovingAverage::new(Self::SHORT_EMA_PERIOD),
            long_ema: ExponentialMovingAverage::new(Self::LONG_EMA_PERIOD),
            ..Default::default()
        };

        strategy
    }

}

impl Strategy for SimpleStrategy {
    fn start(&self, broker: &Broker) {
        info!("Starting strategy");

        let candle_count = 1; // self.long_ema.num_data_missing() + 1;
        let candles = broker.candles(candle_count, self.settings.candle_size);

        for c in candles {
            self.short_ema.update(c);
            self.long_ema.update(c);
        }

        self.set_trading_status(true);
    }

    fn stop(&self) {
        info!("Stopping strategy");
        self.set_trading_status(false);
    }

    fn update(&self, broker: &Broker, datapoint: DataPoint) {
        // Timestopper
        // Check if it is friday night

        let latest_candle = self.current_candle.get().unwrap();

        // Update indicators
        self.short_ema.update(latest_candle);
        self.long_ema.update(latest_candle);

        self.current_candle.set(None);

        let short_ema_value = self.short_ema.value();
        let long_ema_value = self.long_ema.value();

        if short_ema_value > long_ema_value {
            if broker.current_position() > 0.0 && broker.current_position_side() == MarketTrend::EnterLong {
                return;
            } else {
                self.close_position(broker);
                self.buy(broker);
            }
        }

        if long_ema_value > short_ema_value {
            if broker.current_position() > 0.0 && broker.current_position_side() == MarketTrend::EnterShort {
                return;
            } else {
                self.close_position(broker);
                self.sell(broker);
            }
        }
    }

    fn buy(&self, broker: &Broker) {
        info!("Strategy Buy called. Going long @ {}", self.logging_current_price);

        if !self.trading_enabled() {
            info!("Strategy trading disabled, noop");
            return;
        }

        // Enter long position on instrument
        let units = 2; // self.risk.long_position_size();
        info!("# of units to trade: {}", units);

        if units == 0 {
            info!("Can't trade 0 units. Noop");
            return;
        }

        let _ = broker.buy(units);
    }

    fn sell(&self, broker: &Broker) {
        info!("Strategy sell called. Going short @{}", self.logging_current_price);

        if !self.trading_enabled() {
            info!("Trading disabled, noop");
            return;
        }

        let units = 2; //  self.risk.short_position_size();       
        info!("Number of units to trade: {}", units);

        if units == 0 {
            info!("Can't trade 0 units, nop");
            return;
        }

        let _ = broker.sell(units);
    }

    fn trading_enabled(&self) -> bool {
        self.trading_enabled.get()
    }

    fn set_trading_status(&self, status: bool) {
        self.trading_enabled.set(status);
    }

}

fn try_main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        error!("Wrong argument count");
        info!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    // First argument is filename
    // Create Tester settings
    let broker_settings = FileBrokerSettings {
        filename: args[1].clone(),
        ..Default::default()
    };

    // A broker can be used by multiple experts
    let broker = FileBroker::new(broker_settings);

    info!("Backtest initialized");

    let strategy_settings = StrategySettings { candle_size: 120 };

    let strategy = Box::new(SimpleStrategy::new(strategy_settings));

    strategy.run();

    Ok(())
}

fn main() {
    env_logger::init();
    
    if let Err(e) = try_main() {
        error!("{}", e);

        process::exit(1);
    }
}