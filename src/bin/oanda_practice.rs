extern crate chrono;

use chrono::prelude::*;

extern crate futures;
use futures::{Future, Stream};

extern crate reqwest;

// extern crate hyper;
// use hyper::{Method, Request};
// use hyper::header::{Authorization, ContentType, Bearer};

// extern crate hyper_tls;
// use hyper_tls::HttpsConnector;

// extern crate tokio_core;
// use tokio_core::reactor::Core;

extern crate financial;
// use financial::{Account};

use std::io::{self, Write};
use std::env;
use std::net::TcpStream;

struct OandaClient {
    api_key: String
}

impl OandaClient {
    const URI: &'static str = "https://api-fxpractice.oanda.com/"; 

    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string()
        }
    }  

    // pub fn accounts(&self) -> Vec<Account> {
    //     let mut endpoint = OandaClient::URI.to_string();
    //     endpoint.push_str("/v3/accounts");

    //     let body = reqwest::get(endpoint)?.text()?;

    //     serde_json::from_str(&body).unwrap()
    // }
}

fn try_main() -> Result<(), Box<std::error::Error>> {
    let client = {
        let api_key = env::var("OANDA_API_KEY").unwrap();
        let client = OandaClient::new(&api_key);

        client
    };


    loop {
        // Candlestick update loop

        // Calculators

        // Risk Calculation

        // Make trades?
    }

    Ok(())
}

// fn try_main_hyper() -> Result<(), Box<std::error::Error>> {
//     // Event loop
//     let mut core = Core::new()?;
//     let handle = core.handle();
//     let client = hyper::Client::configure()
//         .connector(HttpsConnector::new(4, &handle)?)
//         .build(&handle);

//     let uri = "https://api-fxpractice.oanda.com/v3/accounts".parse()?;
//     let api_key = env::var("OANDA_API_KEY").unwrap();

//     let mut request = Request::new(Method::Get, uri);
//     request.headers_mut().set(Authorization(Bearer { token: api_key }));
//     request.headers_mut().set(ContentType::json());

//     let work /*: Future<Response> */ = client
//         .request(request)
//         .and_then(|res| {
//             println!("Response: {}", res.status());

//             res.body().for_each(|chunk| {
//                 io::stdout()
//                 .write_all(&chunk)
//                 .map_err(From::from)
//             })
//         });

//     // Run future in work until success/failure
//     core.run(work)?;

//     Ok(())
// }

fn main() {

    try_main().unwrap();
    // Get time
    // Eastern Daylight Time - -04:00
    // let est = chrono::FixedOffset::west(4 * 60 * 60);
    // let start_of_day = chrono::NaiveTime::from_hms(8, 0, 0);
    // let start_of_day = UTC::today().naive_utc().and_time(start_of_day);
    // let start_of_day = est.from_local_datetime(&start_of_day)
    //     .unwrap()
    //     .with_timezone(&UTC);   
    //let client = Client::new(&url, &key);

    /*
    let mut results = client.pricing_for("EUR_USD".to_string(), start_of_day)
        .with_include_first(false)
        .execute();

    while result.candles.len() > 0 {
        for candle in &result.candles {
            println!("Time stamp: {}", candle.time);
            println!("-------------------------------------------------------");
            println!(" Open: {}", candle.mid.as_ref().unwrap().o);
            println!(" High: {}", candle.mid.as_ref().unwrap().h);
            println!("  Low: {}", candle.mid.as_ref().unwrap().l);
            println!("Close: {}", candle.mid.as_ref().unwrap().c);
            println!("#######################################################");
        }

        // Load next set of candles

        results = client.pricing_for("EUR_USD".to_string(), results.candles.last().unwrap().time)
            .with_include_first(false)
            .execute();
    }
    */
}