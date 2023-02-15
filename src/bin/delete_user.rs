use diesel::prelude::*;
use std::env::args;
use stock_portfolio_manager::*;

fn main() {
    use self::schema::users::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(users.filter(user_name.like(pattern)))
        .execute(connection)
        .expect("Error deleting users");

    println!("Deleted {} users", num_deleted);
}
