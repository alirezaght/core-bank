#[macro_use]
extern crate diesel;

mod db;

use chrono::format::Item::Error;
use diesel::prelude::*;
use uuid::Uuid;

mod schema;
mod models;
mod core;

use db::dao::dao_transaction;
use schema::t_transaction::dsl;
use crate::core::impl_account::*;
use crate::core::impl_address::*;
use crate::core::impl_transaction::*;
use crate::core::impl_audit_account::*;
use crate::core::impl_audit_transaction::*;

use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::{RequestMiddleware, RequestMiddlewareAction, Response, ServerBuilder};
use jsonrpc_http_server::hyper::{Body, Request};


use crate::core::address::Address;


// struct Security;
// impl RequestMiddleware for Security {
//    fn on_request(&self, request: Request<Body>) -> RequestMiddlewareAction {
//       let headers = request.headers();
//       let auth = headers.get("Authorization");
//       if auth.is_none() {
//          RequestMiddlewareAction::from(Response::service_unavailable("Authentication Failed"))
//       } else {
//          RequestMiddlewareAction::from(Response::service_unavailable("Authentication Failed"))
//       }
//    }
// }

fn main() {

   let mut io = IoHandler::default();
   io.extend_with(CoreAddressImpl.to_delegate());
   io.extend_with(CoreAccountImpl.to_delegate());
   io.extend_with(CoreAccountAuditImpl.to_delegate());

   let server = ServerBuilder::new(io)
       // .request_middleware(Security)
       .threads(3)
       .start_http(&"0.0.0.0:3030".parse().unwrap())
       .unwrap();

   server.wait();

}
