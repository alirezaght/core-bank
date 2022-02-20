use chrono::{DateTime, Utc};
use crate::core::address::Address;
use crate::core::dto::{AccountDTO, TransactionDTO};
use crate::models::{Account, TransactionType};
use crate::db::dao::dao_account;
use crate::db::dao::dao_transaction;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;

pub struct CoreAccountAuditImpl;

#[rpc]
pub trait CoreAuditAccount {
    #[rpc(name = "block_withdraw")]
    fn block_withdraw(&self, address: String, comment: String) -> Result<String>;
    #[rpc(name = "unblock_withdraw")]
    fn unblock_withdraw(&self, address: String, comment: String) -> Result<String>;
    #[rpc(name = "block_deposit")]
    fn block_deposit(&self, address: String, comment: String) -> Result<String>;
    #[rpc(name = "unblock_deposit")]
    fn unblock_deposit(&self, address: String, comment: String) -> Result<String>;
    #[rpc(name = "block")]
    fn block(&self,address: String, comment: String) -> Result<String>;
    #[rpc(name = "unblock")]
    fn unblock(&self, address: String, comment: String) -> Result<String>;
}

impl CoreAuditAccount for CoreAccountAuditImpl {

    fn block_withdraw(&self, address: String, comment: String) -> Result<String> {
        let account = dao_account::findByAddress(address);
        match account { None => Err(Error{
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None
        }),
            Some(mut account) => {
                let seq = account.seq;
                account.seq += 1;
                account.withdraw = false;
                account.comment = Option::from(comment);
                account.created = Utc::now();
                dao_account::save(account);
                Ok("OK".to_string())
            }
        }
    }

    fn unblock_withdraw(&self, address: String, comment: String) -> Result<String> {
        let account = dao_account::findByAddress(address);
        match account { None => Err(Error{
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None
        }),
            Some(mut account) => {
                let seq = account.seq;
                account.seq += 1;
                account.withdraw = true;
                account.comment = Option::from(comment);
                account.created = Utc::now();
                dao_account::save(account);
                Ok("OK".to_string())
            }
        }
    }

    fn block_deposit(&self, address: String, comment: String) -> Result<String> {
        let account = dao_account::findByAddress(address);
        match account { None => Err(Error{
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None
        }),
            Some(mut account) => {
                let seq = account.seq;
                account.seq += 1;
                account.deposit = false;
                account.comment = Option::from(comment);
                account.created = Utc::now();
                dao_account::save(account);
                Ok("OK".to_string())
            }
        }
    }

    fn unblock_deposit(&self, address: String, comment: String) -> Result<String> {
        let account = dao_account::findByAddress(address);
        match account { None => Err(Error{
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None
        }),
            Some(mut account) => {
                let seq = account.seq;
                account.seq += 1;
                account.deposit = true;
                account.comment = Option::from(comment);
                account.created = Utc::now();
                dao_account::save(account);
                Ok("OK".to_string())
            }
        }
    }

    fn block(&self, address: String, comment: String) -> Result<String> {
        let account = dao_account::findByAddress(address);
        match account { None => Err(Error{
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None
        }),
            Some(mut account) => {
                let seq = account.seq;
                account.seq += 1;
                account.withdraw = false;
                account.deposit = false;
                account.comment = Option::from(comment);
                account.created = Utc::now();
                dao_account::save(account);
                Ok("OK".to_string())
            }
        }
    }

    fn unblock(&self, address: String, comment: String) -> Result<String> {
        let account = dao_account::findByAddress(address);
        match account { None => Err(Error{
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None
        }),
            Some(mut account) => {
                let seq = account.seq;
                account.seq += 1;
                account.withdraw = true;
                account.deposit = true;
                account.comment = Option::from(comment);
                account.created = Utc::now();
                dao_account::save(account);
                Ok("OK".to_string())
            }
        }
    }
}