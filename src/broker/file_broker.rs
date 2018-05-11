use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

use chrono::{NaiveDateTime};

use ::{Broker, DataPoint, MarketTrend};

pub struct FileBrokerSettings {
    pub leverage: i32,
    pub filename: String,
    pub account_value: f64
}

impl Default for FileBrokerSettings {
    fn default() -> Self {
        Self {
            leverage: 50,
            filename: String::new(),
            account_value: 100.0
        }
    }
}

struct PlotData {
    raw_price: Vec<f64>,
    sell: Vec<f64>,
    buy: Vec<f64>,
    close: Vec<f64>,
    stop_loss: Vec<f64>,
    trailing_stop: Vec<f64>,
    net_worth: Vec<f64>
}

impl Default for PlotData {
    fn default() -> Self {
        Self {
            raw_price: vec![],
            sell: vec![],
            buy: vec![],
            close: vec![],
            stop_loss: vec![],
            trailing_stop: vec![],
            net_worth: vec![]
        }
    }
}

impl PlotData {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct FileBroker {
    settings: FileBrokerSettings,
    file: RefCell<Option<BufReader<File>>>,
    plot_data: PlotData,
    total_pnl: Cell<f64>,
    net_worth: f64,
    // ticker_subscribers: RefCell<Vec<Rc<RefCell<BrokerClient>>>>,
    position: Cell<f64>,
    position_side: Cell<MarketTrend>,
    balance_instrument: Cell<f64>,
    balance_home_currency: Cell<f64>,
    cash_invested: Cell<f64>,
    current_price: Cell<f64>,
    leverage: i32,
    // balance: ()
    last_entered_price: Cell<f64>,
    // last_update_timestamp: Option<Time>
}

impl Default for FileBroker {
    fn default() -> Self {
        Self {
            balance_instrument: Cell::new(0.0),
            balance_home_currency: Cell::new(0.0),
            cash_invested: Cell::new(0.0),
            current_price: Cell::new(0.0),
            file: RefCell::new(None),
            last_entered_price: Cell::new(0.0),
            leverage: 50,
            net_worth: 0.0,
            plot_data: PlotData::new(),
            position: Cell::new(0.0),
            position_side: Cell::new(MarketTrend::None),
            total_pnl: Cell::new(0.0),
            settings: FileBrokerSettings::default(),
        }
    }
}

impl FileBroker {
    pub fn new(settings: FileBrokerSettings) -> Self {
        Self {
            net_worth: settings.account_value,
            file: RefCell::new(None),
            plot_data: PlotData::new(),
            // ticker_subscribers: RefCell::new(Vec::new()),
            settings,
            ..Default::default()
        }
    }

    
}

impl Broker for FileBroker {
    fn start_price_streaming(&self) {
        if let Some(_) = *self.file.borrow() {
            warn!("File already opened for price streaming");
            return;
        }

        let filename = &self.settings.filename;

        let f = File::open(filename).expect("Failed to open file");
        let bf = BufReader::new(f);

        // let fs::read_tor

        let _ = self.file.replace(Some(bf));
    }

    fn stop_price_streaming(&self) {
        info!("Stopping strategy");
        // Close file

        info!("Total PnL: {} NetWorth: {}", self.total_pnl.get(), self.net_worth());
    }

    /**
     * Add a new client to real time broker data
     */
    // fn subscribe_ticker(&self, subscriber: Rc<RefCell<Broker>>) {
    //     self.ticker_subscribers.borrow_mut().push(subscriber);
    // }

    // Read a datapoint from source
    // Parse datapoint into a candle and send to all subscribers
    fn latest_datapoint(&self) -> Option<DataPoint> {
        // Get a line from file and parse

        if self.file.borrow().is_none() {
            warn!("Backtest has no file opened");
            return None;
        }

        let mut buffer = String::new();

        let num_bytes = self.file.borrow_mut()
            .as_mut()
            .unwrap()
            .read_line(&mut buffer);

        if buffer.is_empty() {
            self.stop_price_streaming();
            return None;
        }

        let tokens: Vec<&str> = buffer.split(';').collect();

        if tokens.len() != 6 {
            error!("Incorrect line in data file");
            self.stop_price_streaming();
            return None;
        }

        // Parse
        let datestr = &tokens[0];
        let open = tokens[1].parse::<f64>().unwrap();
        let high = tokens[2].parse::<f64>().unwrap();
        let low = tokens[3].parse::<f64>().unwrap();
        let close = tokens[4].parse::<f64>().unwrap();
        // let volume = tokens[5].parse::<f64>()?;

        let price = close;

        let timestamp = NaiveDateTime::parse_from_str(datestr, "%Y%m%d %H%M%S").unwrap();

        // Form ticker
        let datapoint = (timestamp, price);
        // self.last_update_timestamp = timestamp;
        // self.current_price = price;

        // Push to all
        // self.create_plot_record('rawproce')
        // self.create_plot_record('net_worth, net_worth);

        Some(datapoint)
    }

    fn is_running(&self) -> bool {
        self.file.borrow().is_some()
    }

    fn sell(&self, units: u64) {
        self.position.set(units as f64);    
        self.position_side.set(MarketTrend::EnterShort);

        let current_price = self.current_price.get();
        let balance_instrument = self.balance_instrument.get() - units as f64;
        let bhc = self.balance_home_currency.get() + (units as f64 * current_price) / self.leverage as f64;

        self.balance_instrument.set(balance_instrument);
        self.balance_home_currency.set(bhc);
        
        self.cash_invested.set(units as f64 * self.current_price.get());
        self.last_entered_price.set(self.current_price.get());
    
        // Create plot record (sell)
    }

    fn buy(&self, units: u64) { 
        self.position.set(units as f64);
        self.position_side.set(MarketTrend::EnterLong);

        let leverage = self.leverage as f64;
        let current_price = self.current_price.get();
        let balance_instrument = self.balance_instrument.get() + units as f64; 
        let bhc = self.balance_home_currency.get() - (units as f64 * current_price) / leverage; 

        self.balance_home_currency.set(bhc);
        self.balance_instrument.set(balance_instrument);

        self.cash_invested.set(units as f64 * current_price);
        self.last_entered_price.set(current_price);

        // Create plot record "buy"
    }

    fn close_position(&self) {
        // Check if positions are open
        // if (instrument not in self.balance) { return; }

        let leverage = self.leverage as f64;
        let cash_invested = self.cash_invested.get();
        let current_price = self.current_price.get();
        let mut bhc = self.balance_home_currency.get();
        let instrument = self.balance_instrument.get();

        let mut realized_pnl;

        if instrument > 0.0 {
            realized_pnl = (instrument.abs() * current_price) - cash_invested;  
            realized_pnl *= leverage;

            bhc += cash_invested / leverage;
        } else {
            realized_pnl = cash_invested - instrument.abs() * current_price;
            realized_pnl *= leverage;
            bhc -= cash_invested / leverage;
        }

        self.balance_home_currency.set(bhc + realized_pnl);
        self.balance_instrument.set(0.0);
        self.position.set(0.0);

        info!("Trade ({}): {:?} from: {} to: {}. Realized PnL: {} NetWorth: {}",
            "TIMESTAMP",
            self.position_side.get(),
            self.last_entered_price.get(),
            current_price,
            realized_pnl,
            self.net_worth()
        );

        self.position_side.set(MarketTrend::None);
        self.total_pnl.set(self.total_pnl.get() + realized_pnl);

        // Create plot record ('close') 
    }

    /** Getters */
    fn current_position(&self) -> f64 {
        self.position.get()
    }

    fn current_position_side(&self) -> MarketTrend {
        self.position_side.get()
    }

    fn net_worth(&self) -> f64 {
        let mut net_worth = 0.0;

        net_worth += self.balance_home_currency.get();
        net_worth += self.balance_instrument.get() * self.current_price.get() / self.leverage as f64;

        net_worth
    }
}
