extern crate diesel;
extern crate tutorial;

use diesel::prelude::*;
use tutorial::*;

fn main() {
  use schema::posts::dsl::*;

  let connection = tutorial::connect();
  let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<models::Post>(&connection)
    .expect("Error loading posts");

  println!("Displaying {} posts", results.len());

  for post in results {
    println!("{}", post.title);
    println!("----------------");
    println!("{}", post.body);
  }
}
