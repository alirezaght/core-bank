use bigdecimal::BigDecimal;
use crate::core::{CoreAddress, CoreImpl, CoreTransaction};
use crate::core::address::Address;
use crate::core::dto::TransactionDTO;

impl CoreTransaction for CoreImpl {
    fn list_transactions(&self, address: String, last: i32) -> Vec<TransactionDTO> {
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