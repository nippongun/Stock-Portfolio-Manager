extern crate diesel;
extern crate stock_portfolio_manager;

use anyhow::Result;
use diesel::SqliteConnection;
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
    let connection: &mut SqliteConnection = &mut stock_portfolio_manager::establish_connection();
    let api_key = login(connection);
    let mut api_manager = ApiManager::new(&api_key);

    loop {
        sleep(Duration::from_secs(100));
    }
}

fn login(connection: &mut SqliteConnection) -> String {
    let mut user_name = String::new();
    println!("Enter the user name:");
    stdin().read_line(&mut user_name).unwrap();

    fetch_api_key(connection, &user_name.trim_end())
}
