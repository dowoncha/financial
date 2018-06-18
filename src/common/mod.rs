use chrono::prelude::*;

mod bar;
pub use self::bar::*;

mod data_series;
pub use self::data_series::*;

mod dispatcher;
pub use self::dispatcher::*;

mod observer;
pub use self::observer::*;

pub type Instrument = &'static str;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MarketTrend {
    None,
    Bull,
    Bear,
    EnterLong,
    EnterShort,
    ExitLong,
    ExitShort,
    StopLong,
    StopShort,
    NoStop
}

impl Default for MarketTrend {
    fn default() -> Self {
        MarketTrend::None
    }
}

// #[derive(Copy, Clone)]
// pub struct Candle {
//     pub open: f64,
//     pub high: f64,
//     pub low: f64,
//     pub close: f64,
//     pub open_time: DateTime<Utc>,
//     pub close_time: DateTime<Utc>,
//     is_closed: bool
// }

// impl Default for Candle {
//     fn default() -> Self {
//         Self {
//             open_time: Utc::now(),
//             close_time: Utc::now(),
//             open: 0.0,
//             high: 0.0,
//             low: 0.0,
//             close: 0.0,
//             is_closed: false
//         }
//     }
// }

// impl Candle {
//     pub fn new(open_time: DateTime<Utc>, close_time: DateTime<Utc>) -> Self {
//         Self {
//             open_time,
//             close_time,
//             ..Default::default()
//         }
//     }

//     pub fn reset_price(&mut self, price: f64) {
//         self.open = price;
//         self.close = price;
//         self.high = price;
//         self.low = price;
//     }     

//     fn update_data(&mut self, price: f64) {
//         // First data point
//         if self.high == 0.0 && self.low == 0.0 && self.open == 0.0 && self.close == 0.0 {
//             self.reset_price(price);
//             self.is_closed = false;
//             return;
//         } 

//         self.close = price;
//         self.high = self.high.max(price);
//         self.low = self.low.max(price);
//     } 
// }

// impl Indicator for Candle {
//     fn update(&self, datapoint: DataPoint) {
//         if self.close_time < self.open_time {
//             // self.reset_price(0.0);
//             // self.is_closed = false;
//             return;
//         }

//         // Validate datapoint

//         let current_timestamp = datapoint.0;
//         let price = datapoint.1;

//         // if current_timestamp >= self.close_time {
//         //     self.is_closed = true;
//         // } 

//         // if current_timestamp <= self.close_time && current_timestamp >= self.open_time {
//         //     self.update_data(price);
//         // }
//     }
// }

pub type DataPoint = (NaiveDateTime, f64);

