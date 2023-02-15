use std::io::{stdin, Read};
use stock_portfolio_manager::*;

fn main() {
    let connection = &mut establish_connection();

    let mut user_name = String::new();
    let mut api_key = String::new();

    println!("Enter the user name:");
    stdin().read_line(&mut user_name).unwrap();

    println!("Enter user's asscoiated API key:");
    stdin().read_to_string(&mut api_key).unwrap();

    create_user(connection, &user_name.trim_end(), &api_key.trim_end());
}
