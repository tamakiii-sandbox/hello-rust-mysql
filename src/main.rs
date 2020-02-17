#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::diesel;
use rocket_contrib::databases::r2d2::PooledConnection;

#[database("mysql")]
struct DatabaseConnection(diesel::mysql::MysqlConnection);
// struct DatabaseConnection(diesel::MysqlConnection);

#[get("/")]
fn index (conn: DatabaseConnection) -> String {
    // `rocket_contrib::databases::r2d2::PooledConnection<rocket_contrib::databases::diesel::r2d2::ConnectionManager<rocket_contrib::databases::diesel::MysqlConnection>>` in the current scope
    let query = "SELECT 1; SELECT 2; SELECT 3;";
    // conn.batch_execute(query)
    // format!("{:?}", conn.0.batch_execute("SELECT 1; SELECT 2; SELECT 3")
}

fn main() {
    rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .mount("/", routes![index])
        .launch();
}
