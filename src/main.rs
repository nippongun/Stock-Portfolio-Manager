use anyhow::Result;
use std::env;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use stock::Stock;

mod api_manager;
mod stock;
use api_manager::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut request_counter: i32 = 0;
    // Command args
    // get time
    // load configs
    // load api key

    let mut now: u64;
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let mut api_manager = ApiManager::new(&api_key);
    let ticker = String::from("IBM");
    let stock: Stock = Stock::create_stock(ticker);
    api_manager.stocks.push(stock);
    println!("Stock:{}", api_manager.stocks[0].ticker);
    // loop
    loop {
        // get current time
        now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // poll stocks
        // push stocks to db
        // sleep for duration
        request_counter = request_counter + 1;
        println!("Time: {}, Counter:{}", now, request_counter);

        api_manager.update_stocks().await?;
        sleep(Duration::from_secs(100));
    }
}
