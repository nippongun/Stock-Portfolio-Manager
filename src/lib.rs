#[cfg(test)]
mod test {}

use self::models::{NewPost, Post};
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

pub fn create_post<'a>(connection: &mut SqliteConnection, title: &'a str, body: &'a str) {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
        published: &0,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(connection)
        .expect("Error saving new post");

    let results = posts::table
        .filter(posts::dsl::title.like(format!("%{}%", new_post.title)))
        .load::<Post>(connection)
        .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }
}
