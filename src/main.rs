#[macro_use]
extern crate diesel;

mod db;

use diesel::prelude::*;

mod schema;
mod models;

use schema::t_transaction::dsl::*;

fn main() {
    let connection = db::establish_connection();

    let count = t_transaction.count();
    let result = count.get_result::<i64>(&connection);

    println!("{}", result.unwrap());
}
