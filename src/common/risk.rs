use std::rc::Rc;


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
