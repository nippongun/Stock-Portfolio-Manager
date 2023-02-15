use super::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Debug, Insertable)]
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
