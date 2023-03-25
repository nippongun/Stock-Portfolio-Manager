use super::models::{NewStock, Stock};
use crate::database::schema::stocks;
use diesel::prelude::*;

/// @author: <simppabauer@gmail.com>
///
/// This file is exclusively calling the Diesel API for the Stock model. Therefore, only 'getters' and 'setters' are permitted.

pub fn create_stock<'a>(
    connection: &mut SqliteConnection,
    user_id: &'a i32,
    ticker: &'a String,
    amount: &'a i32,
    purchase_price: &'a f32,
    current_price: &'a f32,
) {
    use crate::database::schema::stocks::table;

    let new_stock = NewStock {
        user_id: user_id,
        ticker: ticker,
        amount: amount,
        purchase_price: purchase_price,
        current_price: &0.00,
    };

    diesel::insert_into(table)
        .values(&new_stock)
        .execute(connection)
        .expect("Error saving new stock");

    let results = stocks::table
        .filter(stocks::dsl::ticker.like(format!("%{}%", new_stock.ticker)))
        .load::<Stock>(connection)
        .expect("Error getting new stock");

    for result in results {
        println!("{:?}", result);
    }
}

pub fn get_all_tickers_from_user<'a>(
    connection: &mut SqliteConnection,
    user_id: &'a i32,
) -> Vec<String> {
    stocks::table
        .select(stocks::ticker)
        .filter(stocks::user_id.eq(user_id))
        .load::<String>(connection)
        .expect("Error loading data")
}

pub fn get_all_stocks_from_user<'a>(
    connection: &mut SqliteConnection,
    user_id: &'a i32,
) -> Vec<Stock> {
    stocks::table
        .filter(stocks::user_id.eq(user_id))
        .load::<Stock>(connection)
        .expect("Error loading data")
}
