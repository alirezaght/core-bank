pub mod dto;
pub mod address;
mod impl_address;
mod impl_account;
mod impl_transaction;
mod impl_audit_account;
mod impl_audit_transaction;

use dto::{AccountDTO, TransactionDTO};
use bigdecimal::BigDecimal;
use crate::core::address::Address;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait CoreAddress {
    #[rpc(name = "create_address")]
    fn create_address(&self, public_key: Option<Box<[u8]>>) -> Result<Address>;
}

pub trait CoreAccount {
    fn store_account(&self, address: String, detail: String, withdraw: bool, deposit: bool, comment: String);
    fn get_account(&self, address: String) -> Option<AccountDTO>;
}

pub trait CoreTransaction {
    fn list_transactions(&self, address: String, last: i32) -> Vec<TransactionDTO>;
    fn deposit(&self, address: String, amount: BigDecimal, comment: String, description: String);
    fn withdraw(&self, address: String, amount: BigDecimal, comment: String, description: String);
    fn transfer(&self, from: String, to: String, amount: BigDecimal, comment: String, description: String);
}

pub trait CoreAuditAccount {
    fn block_withdraw(&self, address: String, comment: String);
    fn unblock_withdraw(&self, address: String, comment: String);
    fn block_deposit(&self, address: String, comment: String);
    fn unblock_deposit(&self, address: String, comment: String);
    fn block(&self,address: String, comment: String);
    fn unblock(&self, address: String, comment: String);
}

pub trait CoreAuditTransaction {
    fn lock(&self, address: String, amount: BigDecimal, comment: String, description: String);
    fn unlock(&self, address: String, amount: BigDecimal, comment: String, description: String);
}


pub struct CoreImpl;



