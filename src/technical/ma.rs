use std::cell::{Cell,RefCell};
use std::collections::VecDeque;

use super::{Indicator};
use ::Candle;

/**
 * Switch data to Vec<Candle>
 **/

#[derive(Default)]
pub struct SimpleMovingAverage {
    value: Cell<f64>,
    period: usize,
    data: Vec<f64> 
}

// impl Default for SimpleMovingAverage { 
//     fn default() -> Self {
//         Self {
//             value: Cell::new(0.0),
//             period: 1,
//             data: Vec::new()
//         }
//     }
// }

impl SimpleMovingAverage {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            ..Default::default()
        }
    } 
}

impl Indicator for SimpleMovingAverage {
    // TODO: Data is candlestick.close
    fn update(&self, data: f64) {
        let data = data.close;

        if self.data.len() >= self.period {
            self.value += (data - self.data[0]) / self.data.len() as f64;

            // Use Deque?
            // Remove outdated datapoint from storage
            self.data.pop();
        } else {
            // Not enoughd ata points. Compute cum moving average
            self.value += (data - self.value) / self.data.len() as f64;
        }

        self.data.push(data);
    }
}

pub struct ExponentialMovingAverage {
    value: Cell<f64>,
    period: usize,
    data: RefCell<VecDeque<f64>>
}

impl ExponentialMovingAverage {
    pub fn new(period: usize) -> Self {
        Self {
            value: Cell::new(0.0),
            period,
            data: RefCell::new(VecDeque::new())
        }
    }

    pub fn value(&self) -> f64 {
        self.value.get()
    }
}

impl Indicator for ExponentialMovingAverage {
    fn update(&self, data: Candle) {
        let data = data.close;

        let data_len: usize = self.data.borrow().len();
        let smoothing = 2.0 / (data_len + 1) as f64;

        let value = self.value.get();
        if data_len >= self.period {
            self.value.set(value + smoothing * (data - value));
            
            // Pop oldest datapoint
            self.data.borrow_mut().pop_front();
        } else {
            // Not enought data, use simple average rather than smoothing
            self.value.set(value + (data - value) / (data_len + 1) as f64);
        }

        self.data.borrow_mut().push_back(data);
    }
}