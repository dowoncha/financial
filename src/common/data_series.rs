use common::{Bar, Event};

use chrono::NaiveDateTime;

use std::collections::VecDeque;

const DEFAULT_MAX_LEN: usize = 1024;

pub struct SequenceDataSeries<T> {
    new_value_event: Event,
    values: VecDeque<T>,
    datetimes: VecDeque<NaiveDateTime>
}

impl<T> SequenceDataSeries<T> {
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        assert!(n <= DEFAULT_MAX_LEN);

        Self {
            new_value_event: Event::new(),
            values: VecDeque::new(),
            datetimes: VecDeque::new()
        }
    }

    pub fn append(&mut self, value: T) {
        self.append_with_datetime(None, value);
    }

    pub fn append_with_datetime(&mut self, datetime: NaiveDateTime, value: T) {
        // if datetime.is_some() && !self.datetimes.len().is_empty() && self.datetimes.last().unwrap() >= datetime.unwrap();

        assert!(self.datetimes.len() == self.values.len());

        self.datetimes.push_back(datetime);
        self.values.push_back(value);

    }
}

pub struct BarDataSeries {
    base: SequenceDataSeries<f64>,
    open_ds: SequenceDataSeries<f64>,
    close_ds: SequenceDataSeries<f64>,
    high_ds: SequenceDataSeries<f64>
}

impl BarDataSeries {
    pub fn new(n: usize) -> Self {
        Self {
            base: SequenceDataSeries::new(n),
            open_ds: SequenceDataSeries::new(n),
            close_ds: SequenceDataSeries::new(n),
            high_ds: SequenceDataSeries::new(n)
        }
    }

    pub fn append(&self, bar: impl Bar) {
        // self.append_with_datetime(bar.datetime(), bar);
    }

    pub fn append_with_datetime(&self, datetime: NaiveDateTime, bar: impl Bar) {

    }
}