use broker::{Broker};
use common::{Bars, Dispatcher};

pub struct BaseStrategy {
    broker: Box<Broker>,
    dispatcher: Dispatcher
}

    // fn market_order()
    // fn limit_order
    // fn stop_order
    // fn stop_limit_order
    // fn enter_long
    /// Generate a MarktOrder to enter a short position
    // fn_enter_short

    // fn enter_short_limit
    // fn enter_long_stop
impl BaseStrategy {
    pub fn new(/* feed */ broker: impl 'static + Broker) -> Self {
        let dispatcher = Dispatcher::new();


        Self {
            broker: Box::new(broker),
            dispatcher
        }
    }
}

/// Implement for trading logic
pub trait Strategy {
    fn base(&self) -> &BaseStrategy;

    fn dispatcher(&self) -> &Dispatcher {
        &self.base().dispatcher
    }

    fn start(&self /* position */);

    fn stop(&self) { self.dispatcher().stop() }

    /// Override to get notified when strategy starts executing
    fn on_start(&self) { }

    /// Override to get notified when strategy finished executing
    fn on_finish(&self, bars: &Bars) { }

    /// Override to get notified when there are no events
    fn on_idle(&self) { }

    fn on_bars(&self, bars: &Bars);

    // fn update(&self, broker: &Broker, datapoint: DataPoint);

    fn buy(&self, broker: &Broker);

    fn sell(&self, broker: &Broker);

    fn close_position(&self, broker: &Broker) {
        info!("Closing position, and all stops");

        if self.trading_enabled() {
            info!("Trading disabled, doing nothing");
            return;
        } 

       // Return result? 
        // let _ = broker.close_position();
    }

    fn pause_trading(&mut self) { 
        info!("Pausing strategy");
        self.set_trading_status(false);
    }

    fn resume_trading(&mut self) {
        info!("Resuming strategy");
        self.set_trading_status(true);
    }

    fn trading_enabled(&self) -> bool {
        false
    }

    fn set_trading_status(&self, status: bool);

    fn run(&self) -> Result<(), &str> {
        self.base().dispatcher.run();

        // if let Some(current_bars) = self.base().bar_feed.current_bars() {
        //     self.on_finish(current_bars)
        // } else {
        //     Err("Feed was empty")
        // }

        Ok(())
    }

    fn on_enter_ok(&self/* , position */) { }


}

