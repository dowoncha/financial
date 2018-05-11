//!
//! Library to backtest/execute trading strategies
//! Focuses on forex but should be able to be used for most instruments
//! Based off of PyAlgoTrade
//! 6 main components
//! 
//! Strategies implement trading logic
//! Feeds are data providing abstractions
//! Brokers are responsible for executing orders
//! Data Series are an abstraction to manage time series
//! Technicals are filters to make calculations using data series
//! LATE FEATURE: Optimizier used for horizontal scaling 

#[macro_use]
extern crate log;

extern crate chrono;

mod broker;
mod common;
mod feed;
mod strategy;
mod technical;