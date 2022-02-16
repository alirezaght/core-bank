use crate::core::{CoreAccount, CoreAddress, CoreImpl};
use crate::core::address::Address;
use crate::core::dto::AccountDTO;

impl CoreAccount for CoreImpl {
    fn store_account(&self, address: String, detail: String, withdraw: bool, deposit: bool, comment: String) {
        todo!()
    }

    fn get_account(&self, address: String) -> Option<AccountDTO> {
        todo!()
    }
}