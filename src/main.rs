#[macro_use]
extern crate diesel;

mod db;

use diesel::prelude::*;
use uuid::Uuid;

mod schema;
mod models;
mod core;

use db::dao::dao_transaction;
use schema::t_transaction::dsl;
use crate::core::create_address;

fn main() {
   let address = create_address(None);
   println!("{:?}", address.address);
   println!("{:?}", address.is_valid());

}
