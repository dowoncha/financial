use common::{Dispatcher, DispatcherPriority};

pub use self::order::{
    Order,
    Type as OrderType,
    Action as OrderAction
};


mod order {

    pub type Quantity = i32;

    pub struct Order<S> {
        id: String,
        _type: Type,
        action: Action,
        state: S
    }

    impl<S> Order<S> {
        /// Creates a market order
        /// Market order is an order to buy o sell a stock at the best available price
        /// on_close - True if order should be filled as close to the closing price as possible
        fn create_market_order(action: Action, instrument: &str, quantity: Quantity, on_close: bool) -> Order<Initial> {
            unimplemented!()
        }

        // create_limit_order

        //create_stop_order 
        
        //create_stop_limit_order 

        // 
    }

    pub enum Type {
        Market,
        Limit,
        Stop,
        StopLimit
    }

    pub enum Action {
        Buy,
        BuyToCover,
        Sell,
        SellShort
    }

    impl From<Order<Initial>> for Order<Submitted> {
        fn from(order: Order<Initial>) -> Order<Submitted> {
            order.state = Submitted {} ;
            order
        }
    }

    struct Initial {
    }

    struct Submitted {
    }
}


pub trait Broker {
    fn dispatch_priority(&self) -> DispatcherPriority {
        DispatcherPriority::Broker
    }

    fn active_orders(&self) -> &[Order];

    fn submit_order(&self, order: Order) -> Result<(), &str>;

    fn cancel_order(&self, order_id: &str) -> Result<(), &str>;

    // fn latest_datapoint(&self) -> Option<DataPoint>;

    // fn subscribe_ticker(&self, subscriber: Rc<RefCell<BrokerClient>>);

    // fn update_subscribers(&self);
}
