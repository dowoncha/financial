/**
 * TOPICS
 * 1. Calculate simple moving averages
 * 2. Compute exponential moving averages
 * 3. Calculate volatility
 * 4. Computing orrelation of equity instruments
 * 5. Modeling and calculating fundamental indicators
 * 
 */

/**
 * Problem: Given a particular equity investment, determine the simple moving avg,
 * and the exponential moving avg for a sequence of closign prices
 */
pub struct MACalculator {
    prices: Vec<f64>,
    periods: usize
}

impl MACalculator {
    pub fn new(periods: usize) -> Self {
        Self {
            periods,
            prices: Vec::new()
        }
    }

    pub fn add_price_quote(&mut self, close: f64) {
        self.prices.push(close);
    }

    pub fn calc_ma(&self) -> Vec<f64> {
        calculate_moving_average(&self.prices, self.periods)
    }

    pub fn calc_ema(&self) -> Vec<f64> {
        calculate_exp_moving_average(&self.prices, self.periods)        
    }
}

/** NOTES **/
// len(prices) is us the initial value for the ema
// There are different implementations to init the sequence, but they all converge
// EMA Multiplier => r = 2 / (N + 1)
fn calculate_exp_moving_average(prices: &[f64], n_periods: usize) -> Vec<f64> {
    let mut ema: Vec<f64> = Vec::new();

    // Factor gives more weight to newer values
    // More responsive to changes in the observed values
    // Can indicate new trends sooner and with better accuracy
    let multiplier = 2.0 / (n_periods + 1) as f64;

    let ma = calculate_moving_average(prices, n_periods);
    ema.push(*ma.first().unwrap());

    for i in (n_periods + 1)..prices.len() {
        let val = ( 1.0 - multiplier ) * *ema.last().unwrap() + multiplier * prices[i];
        ema.push(val);
    }

    ema
}

fn calculate_moving_average(prices: &[f64], n_periods: usize) -> Vec<f64> {
    let mut ma = Vec::new();
    let mut sum = 0.0;

    for i in 0..prices.len() {
        sum += prices[i];

        if i >= n_periods {
            ma.push(sum / n_periods as f64);
            sum -= prices[i - n_periods];
        }
    } 
    
    ma
}