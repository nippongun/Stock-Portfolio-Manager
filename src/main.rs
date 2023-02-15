extern crate diesel;
extern crate stock_portfolio_manager;

use anyhow::Result;
use std::io::stdin;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use stock::Stock;
use stock_portfolio_manager::fetch_api_key;

mod api_manager;
mod database;
mod stock;
use api_manager::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut request_counter: i32 = 0;

    let mut now: u64;
    let api_key = login();

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

fn login() -> String {
    let connection = &mut stock_portfolio_manager::establish_connection();
    let mut user_name = String::new();
    println!("Enter the user name:");
    stdin().read_line(&mut user_name).unwrap();

    fetch_api_key(connection, &user_name.trim_end())
}
