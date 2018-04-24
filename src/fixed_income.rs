/** Architecture Notes **/
// Do I need separate calculator struct?
// OOP - have responsibilities unified under very well defined interfaces:w

// How to check float equality?
// How should money be represented?

// General Principles
// 1. Money in your pocket today is more valuable than the same money received in the fuutre
// 2. 

#[derive(Debug, Copy, Clone)]
pub struct InterestRateCalculator {
    rate: f64
}

impl InterestRateCalculator {
   pub fn new() -> Self {
      Self {
          rate: 0.0
      } 
   }

   pub fn set_rate(&mut self, rate: f64) {
       self.rate = rate;
   }

    #[inline]
    pub fn single_period(&self, value: f64) -> f64 {
        value * ( 1.0 + self.rate )
    }
}

#[test]
fn test_single_period_interest() {
    let mut irc = InterestRateCalculator::new();
    
    irc.set_rate(0.08);
    // assert_eq!(irc.single_period(10000.0), 10800);
}

#[derive(Debug, Copy, Clone)]
pub struct CompoundIntRateCalculator {
    rate: f64
}

impl CompoundIntRateCalculator {
    pub fn new() -> Self {
        Self {
            rate: 0.0
        }
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
    }


    /**
     * Discrete Compounding
     * V = P * ( 1 + R ) ^ N
     */ 
    pub fn multiple_period(&self, value: f64, num_periods: i32) -> f64{
        value * (1.0 + self.rate).powf(num_periods as f64)
    }

    /**
     * Continuous Compounding
     * V = P * e^(RN)
     */
    pub fn continuous_compound(&self, value: f64, num_periods: i32) -> f64{
        value * (self.rate * num_periods as f64).exp()
    }
}

/**
 * CASH FLOW are the basic tool for comparing two or more fixed income investments.
 * - Establishes sequence of cash transfers between two interested parties
 * PRESENT VALUE of a payment in the future needs to be discounted by the interest rate that would be applied to that same value.
 * - Formula for PV of a future payment
 * - PV = FV / (1 + R)^N
 * - Note: Inverse of compound interest rate
 * DISCOUNTING is the inverse concept to COMPOUNDING
 */

#[derive(Debug)]
pub struct CashFlowCalculator {
    payments: Vec<f64>,
    time_periods: Vec<i32>,
    rate: f64
}

impl CashFlowCalculator {
    pub fn new(rate: f64) -> Self {
        Self {
            payments: Vec::new(),
            time_periods: Vec::new(),
            rate
        }
    }

    /**
     * Add new payments to the desired cash flow
     */
    pub fn add_payment(&mut self, value: f64, time_period: i32) {
        self.payments.push(value);
        self.time_periods.push(time_period);
    }

    // Compute PV for whole cash flow
    pub fn present_value(&self) -> f64 {
        /** Optimization ideas:
         * 1. Compare to indexing for loop
         */
        let mut total = 0.0;

        for i in 0..self.payments.len() {
            total += self.single_payment_pv(self.payments[i], self.time_periods[i]);
        }

        total
    }

    /**
     * Compute pv for a single payment
     */
    fn single_payment_pv(&self, fv: f64, time_period: i32) -> f64 {
        let pv = fv / (1.0 + self.rate).powf(time_period.into());
        println!("Single PV: {}", pv);

        pv
    }
}
/**
 * Most bonds are paid off in a time period b/w 5 ~ 30 years
 */
#[derive(Debug, Clone)]
pub struct BondCalculator {
    issuer: String,        // Issuer
    principal: f64,
    coupon: f64,
    n_periods: usize        // Defined in years
}

impl BondCalculator {
    pub fn new(issuer: &str, n_periods: usize, principal: f64, couponValue: f64) -> Self {
        Self {
            issuer: issuer.to_string(),
            principal,
            coupon: couponValue,
            n_periods
        }
    }

    pub fn interest_rate(&self) -> f64 {
        self.coupon / self.principal
    }
}

#[test]
fn test_bond_calculator() {
    let bc = BondCalculator::new("xyz", 20, 100000.0, 5000.0);

    assert_eq!(bc.interest_rate(), 0.05f64);
}