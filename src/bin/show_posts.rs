extern crate diesel;
extern crate tutorial;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

pub fn connect() -> MysqlConnection {
  let url = "mysql://usr:pass@mysql/hello";

  MysqlConnection::establish(url)
    .expect(&format!("Failed to connect: {}", url))
}

fn main() {
  use tutorial::schema::posts::dsl::*;
  use tutorial::models;

  let connection = connect();
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
