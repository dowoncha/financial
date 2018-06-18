use common::{Bar, Bars, Frequency, Instrument, BarDataSeries, Subject};

use std::path::Path;

pub trait Feed : Subject { }

pub trait BarFeed : Feed { 
    fn next_bars(&self) -> Option<Bars>;

    fn current_cars(&self) -> Bars;

    fn last_bar(&self, instrument: Instrument) -> Option<Bar>;

    fn default_instrument(&self) -> Instrument;

    fn data_series(&self) -> BarDataSeries;
}

/// Non real-time Bar feed
/// Hold barfeed 
pub struct MemoryBarFeed {
    started: bool
}

impl MemoryBarFeed {
    
}

pub struct CsvBarFeed {}

impl CsvBarFeed {
    pub fn new(frequency: Frequency, /*timezone*/) -> Self {
        Self {

        }
    }

    pub fn max_capacity(frequency: Frequency, N: usize) -> Self {
        Self {

        }
    }

    pub fn add_bars_from_csv<P: AsRef<Path>>(&mut self, path: P) {

    }
}