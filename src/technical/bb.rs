/**
 * Bollinger Bands
 * Define prevailing high and low prices in a market to characterize trading band
 * of a financial instrument or commodity.
 * 
 * Volatility Indicator
 * 
 * Consist of a N-period moving average,
 * an upper band at MA + Ks, lower at MA - Ks
 * 
 * s = Std Dev
 */

pub struct BollingerBand {
    high_buffer: Vec<f64>,
    mid_buffer: Vec<f64>,
    low_buffer: Vec<f64>,
    std_dev_buffer: Vec<f64> 
    pub N: usize,
    pub K: f32
}

impl Default for BollingerBand {
    fn default() -> Self {
        Self {
            high_buffer: Vec::new(),
            mid_buffer: Vec::new(),
            low_buffer: Vec::new(),
            std_dev_buffer: Vec::new(),
            N: 20,
            K: 2.0
        }
    }
}

impl BollingerBand {
    pub fn new(N: usize, K: f32) -> Self {
        Self {
            N,
            K,
            ..Default::default()
        }
    } 
}

impl Indicator for BollingerBand {
    fn update(&self, datapoint: DataPoint) {

    }
}

pub fn calculate(rates_total: i32, prev_calculated: i32, begin: i32, price: &[f64]) -> i32 {
    let pos = 0;
    // Begin drawing

    // Check for bars count

    // Start calculation

    let mut i = pos;
    while i < rates_total /* && !is_stopped() */ {

        i += 1;
    }

    rates_total
}

fn std_dev(position: i32, price: &[f64], ma_price: &[f64], period: i32) -> f64 {
    let mut std_dev_tmp = 0.0;

    if position < period {
        return std_dev_tmp;
    }

    for i in 0..(period as usize) {
        std_dev_tmp += (price[position as usize - i] - ma_price[position as usize]).powf(2.0);
    }

    std_dev_tmp = (std_dev_tmp / period as f64).sqrt();

    std_dev_tmp
}