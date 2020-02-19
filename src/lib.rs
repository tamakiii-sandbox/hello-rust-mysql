#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

pub mod schema;
pub mod models;

pub fn connect() -> MysqlConnection {
  let url = "mysql://usr:pass@mysql/hello";

  MysqlConnection::establish(url)
    .expect(&format!("Failed to connect: {}", url))
}