use chrono::{DateTime, Utc};
use crate::core::address::Address;
use crate::core::dto::{AccountDTO, TransactionDTO};
use crate::models::{Account, TransactionType};
use crate::db::dao::dao_account;
use crate::db::dao::dao_transaction;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use crate::db::dao::pagination::Pagination;

pub struct CoreAccountImpl;

#[rpc]
pub trait CoreAccount {
    #[rpc(name = "store_account")]
    fn store_account(&self, address: String, detail: String, withdraw: bool, deposit: bool,
                     comment: String) -> Result<String>;
    #[rpc(name = "get_account")]
    fn get_account(&self, address: String) -> Result<AccountDTO>;
    #[rpc(name = "list_account")]
    fn list_account(&self, page: i64, limit: i64) -> Result<Pagination<Account>>;
//TODO:     get account at time
}


impl CoreAccount for CoreAccountImpl {
    fn store_account(&self, address: String, detail: String,
                     withdraw: bool, deposit: bool, comment: String) -> Result<String> {
        let account = Account {
            address,
            detail: Option::from(detail),
            seq: 0,
            withdraw,
            deposit,
            comment: Option::from(comment),
            created: Utc::now(),
        };
        dao_account::save(account);
        Ok("OK".to_string())
    }
    fn get_account(&self, address: String) -> Result<AccountDTO> {
        let account_address = address.clone();
        let account = dao_account::find_by_address(address);
        let transaction = dao_transaction::find_by_account(account_address);
        match account {
            None => Err(Error {
                code: ErrorCode::from(-1),
                message: "Can't find account".to_string(),
                data: None,
            }),
            Some(account) => {
                let mut dto: AccountDTO = AccountDTO::from(account);
                dto.last_tx = match transaction {
                    None => None,
                    Some(transaction) =>
                        Option::from(TransactionDTO::from(transaction))
                };
                Ok(dto)
            }
        }
    }

    fn list_account(&self, page: i64, limit: i64) -> Result<Pagination<Account>> {
        Ok(dao_account::list(page, limit))
    }
}