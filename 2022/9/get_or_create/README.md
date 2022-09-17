welcome to the Rust community!

## get or create

You may indeed make code readable by replacing multiple nested matches with a single match that has nested patterns.

For example, we match on `Err(DatabaseError(UniqueViolation, info))` and that grabs all errors that contain a `DatabaseError` variant of Diesel `Error` enum, with inner `UniqueViolation` variant of `DatabaseErrorKind`. We bind the second value within `DatabaseError` to `info`, so we can print the info super easy too. If, for example, the `Error` is something else than `UniqueViolation`, we fall through to the next match arm.

The pattern sublanguage is like a language within a language -- you have to learn it and build your intuition about it.

The result of our effort is super readable:
```rust
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
```

I had an idea that you may only build one query, which would use `ON CONFLICT`, and kill two birds with one stone. Unfortunately, Diesel dsl does not seem to support `ON CONFLICT (...) DO NOTHING RETURNING *`.

## Other concerns

Syntax nitpick:

```rust
let user = schema::users::dsl::users
    .filter(email.eq(user_email))
    .first(&mut conn)
    .unwrap();
return user;
```
You may just return the user value directly, replacing the above code with this:
```rust
schema::users::dsl::users
    .filter(email.eq(user_email))
    .first(&mut conn)
    .unwrap()
```

## Result

The result is available on my github: https://github.com/pczarn/codereview/tree/81d3fcddd3921bf1b4df4bb347be5dcad3de743f/2022/9/get_or_create

I cleaned up your code, migrated to sqlite for local testing and this is what I got:

```rust
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
