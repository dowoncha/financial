pub type TradeID = String;

pub enum TradeState {
    Open,
    Closed,
    // Trade will be closed as soon as the trade's
    // instrument becomes tradeable
    CloseWhenTradeable
}

// TradeStateFilter

// TradeSpecifier

pub struct Trade {
    id: TradeID,
    // instrument: InstrumentName,
    // price,
    // open_time
    state: TradeState,
}

// TradeSummary

// CalculatedTradeState

// TradePL