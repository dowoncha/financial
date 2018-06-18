struct BacktestingStrategy {
    base: BaseStrategy
}

impl BacktestingStrategy {
    pub fn new(/* bar_feed: BarFeed*/) -> Self {
        let bar_feed = false;
        let broker = BacktestingBroker(1000000, bar_feed);
        
        Self {
            base: BaseStrategy::new(broker);
        }
    }

    pub fn with_broker(broker: impl Broker) -> Self {
        Self {
            base: BaseStrategy::new(broker);
        }
    }
}