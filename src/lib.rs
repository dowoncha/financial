mod fixed_income;
mod equities;

// Chapter 1
pub use self::fixed_income::*;

// Chapter 2
pub use self::equities::*;

use std::rc::Rc;

struct FinancialStatement {
    m_return: f64,
    transactions: Vec<(String, f64)>
}

impl FinancialStatement {
    pub fn new() -> Self {
        Self {
            m_return :0.0,
            transactions: Vec::new()
        }
    }

    pub fn sample() -> Self {
        let mut fs = Self::new();
        fs.add_transaction("IBM", 102.2);
        fs.add_transaction("AAPL", 523.0);

        fs
    }

    pub fn add_transaction(&mut self, security: &str, value: f64) {
        self.transactions.push((security.to_string(), value));
    }
}

#[derive(Copy, Clone)]
enum RiskType {
    AAA,
    AAPlus,
    AA,
    APlus,
    A,
    BPlus,
    B,
    CPlus,
    C
}

struct CreditRisk {
    risk_type: RiskType    
}

impl CreditRisk {
    pub fn rating(&self) -> RiskType {
        self.risk_type
    }
}

struct RiskCalculator {
    credit_risks: Vec<Rc<CreditRisk>>
}

impl RiskCalculator {
    pub fn add_credit_risk(&mut self, risk: Rc<CreditRisk>) {
        self.credit_risks.push(risk);
    }

    pub fn portfolio_max_risk(&self) -> RiskType {
        let risk = RiskType::AAA;

        // for cr in self.credit_risks {
            // if cr.rating() < risk {
            //     risk = cr.rating();
            // }
        // }

        risk
    }
}