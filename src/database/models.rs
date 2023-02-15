use super::schema::stocks;
use super::schema::users;

use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub api_key: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub api_key: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Stock {
    pub id: i32,
    pub user_id: i32,
    pub ticker: String,
    pub purchase_price: f32,
    pub current_price: f32,
}

#[derive(Insertable)]
#[diesel(table_name = stocks)]
pub struct NewStock<'a> {
    pub user_id: &'a i32,
    pub ticker: &'a str,
    pub purchase_price: &'a f32,
    pub current_price: &'a f32,
}
