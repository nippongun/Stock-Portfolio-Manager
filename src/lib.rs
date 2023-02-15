use database::models::{NewUser, User};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod database;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Establish connection to {}", database_url);
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connetcting to Database. URL: {}", database_url))
    //.expect(&format!("Error connencting to {}", database_url))
}

pub fn fetch_api_key<'a>(connection: &mut SqliteConnection, user_name: &'a str) -> String {
    use database::schema::users;

    let data: User = users::table
        .filter(users::dsl::user_name.eq(user_name))
        .first(connection)
        .expect("Error");

    data.api_key
}
