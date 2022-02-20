use bigdecimal::BigDecimal;
use crate::core::address::Address;
use crate::core::dto::TransactionDTO;
use jsonrpc_derive::rpc;


pub struct CoreTransactionImpl;

pub trait CoreTransaction {
    fn list_transactions(&self, address: String, page: i64, limit: i64) -> Vec<TransactionDTO>;
    fn deposit(&self, address: String, amount: BigDecimal, comment: String, description: String);
    fn withdraw(&self, address: String, amount: BigDecimal, comment: String, description: String);
    fn transfer(&self, from: String, to: String, amount: BigDecimal, comment: String, description: String);
}

impl CoreTransaction for CoreTransactionImpl {
    fn list_transactions(&self, address: String, page: i64, limit: i64) -> Vec<TransactionDTO> {
        todo!()
    }

    fn deposit(&self, address: String, amount: BigDecimal, comment: String, description: String) {
        todo!()
    }

    fn withdraw(&self, address: String, amount: BigDecimal, comment: String, description: String) {
        todo!()
    }

    fn transfer(&self, from: String, to: String, amount: BigDecimal, comment: String, description: String) {
        todo!()
    }
}