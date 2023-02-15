use super::models::{NewUser, User};
use crate::database::schema::users;
use diesel::prelude::*;

pub fn create_user<'a>(connection: &mut SqliteConnection, user_name: &'a str, api_key: &'a str) {
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
