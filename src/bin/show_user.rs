extern crate diesel;
extern crate stock_portfolio_manager;

use self::diesel::prelude::*;
use self::models::*;
use self::stock_portfolio_manager::*;

fn main() {
    use stock_portfolio_manager::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.user_name);
    }
}
