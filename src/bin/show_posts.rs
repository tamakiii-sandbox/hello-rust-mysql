#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
// extern crate self as show_posts;

// use self::models::*;
// use self::models::{Post};
use tutorial::*;
use models::{Post};
use diesel::prelude::*;

fn main() {
    // use self::schema::posts::dsl::*;
    use schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

// extern crate diesel_demo;
// extern crate diesel;

// use self::diesel_demo::*;
// use self::models::*;
// use self::diesel::prelude::*;

// fn main() {
//     use diesel_demo::schema::posts::dsl::*;

//     let connection = establish_connection()
//       .expect("Failed to establish connection");

//     let results = posts.filter(published.eq(true))
//         .limit(5)
//         .load::<Post>(&connection)
//         .expect("Error loading posts");

//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{}", post.title);
//         println!("----------\n");
//         println!("{}", post.body);
//     }
// }