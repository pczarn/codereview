extern crate diesel;

mod schema;

use log::warn;

use diesel::prelude::*;
use diesel::result;

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    email: &'a str,
}

#[derive(Queryable)]
struct User {
    id: i32,
    email: String,
}

pub fn get_or_create_user(email: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { email };
    let mut conn = get_connection();

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn);

    match result {
        Ok(user) => return user,
        Err(err) => match err {
            result::Error::DatabaseError(err_kind, info) => match err_kind {
                result::DatabaseErrorKind::UniqueViolation => {
                    warn!(
                        "{:?} is already exists. Info: {:?}. Skipping.",
                        new_user, info
                    );
                    // another query to DB to get existing user by email
                    let user = user_by_email(new_user.email);
                    return user;
                }
                _ => {
                    panic!("Database error: {:?}", info);
                }
            },
            _ => {
                // TODO: decide how to deal with unexpected errors
                return User {
                    id: 0,
                    email: "".into(),
                };
            }
        },
    }
}

pub fn user_by_email(user_email: &str) -> User {
    use crate::schema::users::dsl::*;

    let mut conn = get_connection();

    let user = crate::schema::users::dsl::users
        .filter(email.eq(user_email))
        .first(&mut conn)
        .unwrap();
    return user;
}

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn get_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {

}
