#[macro_use]
extern crate diesel;

mod db;

use diesel::prelude::*;
use uuid::Uuid;

mod schema;
mod models;
use db::dao::dao_transaction;
use schema::t_transaction::dsl;

fn main() {
    let connection = db::establish_connection();

    let count = dsl::t_transaction.count();
    let result = count.get_result::<i64>(&connection);
    let id = uuid::Uuid::parse_str("44935fe3-69b3-44fe-b2de-10ca80e8aa21").unwrap();
    let t = dao_transaction::findByAccount(String::from(""));
    println!("{:?}", t);
}
