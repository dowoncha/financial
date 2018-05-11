pub struct Accounts<'a> {
    pub accounts: Vec<Account<'a>>
}

pub struct Account<'a> {
    pub id: String,
    pub tags: Vec<String>,
//     #[serde(default = "none")]
//     #[serde(skip_deserializing)]
//     pub client: Option<&'a Client<'a>>
}

impl <'a>Account<'a> {
    // pub fn details(&self) -> Details {
    //     let input = self.client().get(format!("accounts/{}", self.id).as_str());
    //     let result: AccountDetails = serde_json::from_str(&input).unwrap();

    //     result.account
    // }

    // pub fn instruments(&self) -> Vec<Instrument> {
    //     let input = self.client().get(
    //         format!("accounts/{}/instruments", self.id).as_str()
    //     );
    //     let result: AccountInstruments = serde_json::from_str(&input).unwrap();

    //     result.instruments
    // }

    // pub fn summary(&self) -> Summary {
    //     let input = self.client().get(
    //         format!("accounts/{}/summary", self.id).as_str()
    //     );
    //     let result: AccountSummary = serde_json::from_str(&input).unwrap();

    //     result.account
    // }

    // fn client(&self) -> &'a Client<'a> {
    //     self.client.expect("Account cannot refer to a client")
    // }
}

