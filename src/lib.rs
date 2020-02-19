#[macro_use] extern crate diesel;
// extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;

pub fn establish_connection() -> MysqlConnection {
    // dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "mysql://usr:pass@mysql/hello";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// #[macro_use]
// extern crate diesel;

// pub mod schema;
// pub mod models;

// use diesel::prelude::*;
// use diesel::mysql::MysqlConnection;
// use self::models::{Post, NewPost};

// pub fn establish_connection() -> ConnectionResult<MysqlConnection> {
//   let database_url = "mysql://usr:pass@mysql/hello";
//   MysqlConnection::establish(&database_url)
// }

// pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> Post {
//     use schema::posts;

//     let new_post = NewPost {
//         title: title,
//         body: body,
//     };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .get_result(conn)
//         .expect("Error saving new post")
// }