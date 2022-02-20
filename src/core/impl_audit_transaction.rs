use bigdecimal::BigDecimal;
use crate::core::address::Address;
use crate::core::dto::TransactionDTO;
use jsonrpc_derive::rpc;


pub struct CoreAuditTransactionImpl;

pub trait CoreAuditTransaction {
    fn lock(&self, address: String, amount: BigDecimal, comment: String, description: String);
    fn unlock(&self, address: String, amount: BigDecimal, comment: String, description: String);
}

impl CoreAuditTransaction for CoreAuditTransactionImpl {
    fn lock(&self, address: String, amount: BigDecimal, comment: String, description: String) {
        todo!()
    }

    fn unlock(&self, address: String, amount: BigDecimal, comment: String, description: String) {
        todo!()
    }
}