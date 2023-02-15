extern crate diesel;
extern crate stock_portfolio_manger;

use self::diesel::prelude::*;
use self::models::*;
use self::stock_portfolio_manger::*;

fn main() {
    use stock_portfolio_manger::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(1))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("{}", post.body);
    }
}
