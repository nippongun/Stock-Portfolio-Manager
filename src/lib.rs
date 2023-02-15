use self::models::{NewUser, User};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Establish connection to {}", database_url);
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connetcting to Database. URL: {}", database_url))
    //.expect(&format!("Error connencting to {}", database_url))
}

pub fn create_user<'a>(connection: &mut SqliteConnection, user_name: &'a str, api_key: &'a str) {
    use schema::users;

    let new_user = NewUser {
        user_name: user_name,
        api_key: api_key,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    let results = users::table
        .filter(users::dsl::user_name.like(format!("%{}%", new_user.user_name)))
        .load::<User>(connection)
        .expect("Error getting new user");

    for result in results {
        println!("{:?}", result);
    }
}

pub fn fetch_api_key<'a>(connection: &mut SqliteConnection, user_name: &'a str) -> String {
    use schema::users;

    let data: User = users::table
        .filter(users::dsl::user_name.eq(user_name))
        .first(connection)
        .expect("Error");

    data.api_key
}
