pub mod dto;
pub mod address;

use dto::{AccountDTO, TransactionDTO};
use bigdecimal::BigDecimal;
use crate::core::address::Address;


pub fn create_address(public_key: Option<&[u8]>) -> Address {
    match public_key { Some(pubK) => Address::from_public(pubK),
        None => Address::create()
    }
}

pub fn store_account(address: String, detail: String, withdraw: bool, deposit: bool, comment: String) {}

pub fn get_account(address: String) -> Option<AccountDTO> {
    None
}

pub fn list_transactions(address: String, last: i32) -> Vec<TransactionDTO> {
    vec![]
}

pub fn deposit(address: String, amount: BigDecimal, comment: String, description: String) {}

pub fn withdraw(address: String, amount: BigDecimal, comment: String, description: String) {}

pub fn transfer(from: String, to: String, amount: BigDecimal, comment: String, description: String) {}

pub fn block_withdraw(address: String, comment: String) {}

pub fn unblock_withdraw(address: String, comment: String) {}

pub fn block_deposit(address: String, comment: String) {}

pub fn unblock_deposit(address: String, comment: String) {}

pub fn block(address: String, comment: String) {}

pub fn unblock(address: String, comment: String) {}

pub fn lock(address: String, amount: BigDecimal, comment: String, description: String) {}

pub fn unlock(address: String, amount: BigDecimal, comment: String, description: String) {}

