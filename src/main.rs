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
use crate::core::{CoreImpl, CoreAddress};
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::ServerBuilder;


use crate::core::address::Address;


fn main() {

   let mut io = IoHandler::default();
   io.extend_with(CoreImpl.to_delegate());

   let server = ServerBuilder::new(io)
       .threads(3)
       .start_http(&"127.0.0.1:3030".parse().unwrap())
       .unwrap();

   server.wait();

}
