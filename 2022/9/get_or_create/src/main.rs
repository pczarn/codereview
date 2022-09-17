extern crate diesel;

mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use log::warn;

use diesel::prelude::*;

use schema::users;

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    email: &'a str,
}

#[derive(Queryable)]
pub struct User {
    id: i32,
    email: String,
}

pub fn get_or_create_user(email: &str) -> User {
    use diesel::result::{Error::DatabaseError, DatabaseErrorKind::UniqueViolation};

    let new_user = NewUser { email };
    let mut conn = get_connection();

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn);

    match result {
        Ok(user) => return user,
        Err(DatabaseError(UniqueViolation, info)) => {
            warn!(
                "{:?} is already exists. Info: {:?}. Skipping.",
                new_user, info
            );
            // another query to DB to get existing user by email
            user_by_email(new_user.email)
        }
        Err(DatabaseError(_, info)) => {
            panic!("Database error: {:?}", info);
        }
        _ => {
            // TODO: decide how to deal with unexpected errors
            User {
                id: 0,
                email: "".into(),
            }
        }
    }
}

pub fn user_by_email(user_email: &str) -> User {
    use schema::users::dsl::*;

    let mut conn = get_connection();

    let user = schema::users::dsl::users
        .filter(email.eq(user_email))
        .first(&mut conn)
        .unwrap();
    return user;
}

pub fn get_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    get_or_create_user("example@example.com");
    get_or_create_user("example@example.com");
}
