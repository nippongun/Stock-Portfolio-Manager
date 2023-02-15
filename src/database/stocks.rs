use crate::database::schema::stocks;

use super::models::{NewStock, Stock};
use diesel::prelude::*;

pub fn create_stock<'a>(
    connection: &mut SqliteConnection,
    user_id: &'a i32,
    ticker: &'a String,
    purchase_price: &'a f32,
    current_price: &'a f32,
) {
    use crate::database::schema::stocks::table;

    let new_stock = NewStock {
        user_id: user_id,
        ticker: ticker,
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
