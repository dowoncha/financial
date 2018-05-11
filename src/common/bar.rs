use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Frequency {
    Trade = -1,
    Second = 1,
    Minute = 60,
    Hour = 60 * 60,
    Day = 24 * 60 * 60,
    Week = 24 * 60 * 60 * 7,
    Month = 24 * 60 * 60 * 31
}

/// A Bar is a summary of the trading activity for a security in a given period.
// Q: What is adjusted value for?
pub trait Bar {
    // get/set use adjusted_value

    // get date_time

    fn open(&self) -> f64;

    fn high(&self) -> f64;

    fn close(&self) -> f64;

    fn low(&self) -> f64;

    fn frequency(&self) -> Frequency;

    fn volume(&self) -> f64;

    fn typical_price(&self) -> f64 {
        (self.high() + self.low() + self.close()) / 3.0
    }

    /// Returns the closeing or adjusted closing price
    fn price(&self) -> f64;
}

struct BasicBar {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    frequency: Frequency
}

impl BasicBar {
    pub fn new(/* datetime */ open: f64, high: f64, low: f64, close: f64, volume: f64, /* adjustedClose */ frequency: Frequency) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            frequency
        }
    }
}

impl Bar for BasicBar {
    fn open(&self) -> f64 {
        self.open
    }
    
    fn high(&self) -> f64 {
        self.high
    }
    
    fn low(&self) -> f64 {
        self.low
    }

    fn close(&self) -> f64 {
        self.close
    }

    fn volume(&self) -> f64 {
        self.volume
    }

    fn frequency(&self) -> Frequency {
        self.frequency
    }

    fn price(&self) -> f64 {
        self.close
    }
}

/// A group of Bar objects
pub type Bars = HashMap<&'static str, Bar>;