use bigdecimal::BigDecimal;
use crate::core::{CoreAddress, CoreAuditAccount, CoreAuditTransaction, CoreImpl, CoreTransaction};
use crate::core::address::Address;
use crate::core::dto::TransactionDTO;

impl CoreAuditTransaction for CoreImpl {
    fn lock(&self, address: String, amount: BigDecimal, comment: String, description: String) {
        todo!()
    }

    fn unlock(&self, address: String, amount: BigDecimal, comment: String, description: String) {
        todo!()
    }
}