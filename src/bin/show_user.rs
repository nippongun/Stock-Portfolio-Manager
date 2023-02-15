use crate::database::schema::users::dsl::*;
use diesel::prelude::*;
use stock_portfolio_manager::database::models::User;
use stock_portfolio_manager::*;

fn main() {
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
