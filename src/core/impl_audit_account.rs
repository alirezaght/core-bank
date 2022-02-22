use chrono::{DateTime, Utc};
use crate::core::address::Address;
use crate::core::dto::{AccountDTO, TransactionDTO};
use crate::models::{Account, TransactionType};
use crate::db::dao::dao_account;
use crate::db::dao::dao_transaction;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use crate::schema::t_account;

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

fn audit_account(address: String, comment: String, withdraw: bool, deposit: bool,
                 set_withdraw: bool, set_deposit: bool) -> Result<String> {
    let account = dao_account::find_by_address(address);
    match account { None => Err(Error{
        code: ErrorCode::from(-1),
        message: "Couldn't find account".to_string(),
        data: None
    }),
        Some(mut account) => {
            let seq = account.seq;
            account.seq += 1;
            if set_withdraw {
                account.withdraw = withdraw;
            }
            if set_deposit {
                account.deposit = deposit;
            }
            account.comment = Option::from(comment);
            account.created = Utc::now();
            dao_account::save(account);
            Ok("OK".to_string())
        }
    }
}

impl CoreAuditAccount for CoreAccountAuditImpl {

    fn block_withdraw(&self, address: String, comment: String) -> Result<String> {
        audit_account(address, comment, false,
                      false, true, false)
    }

    fn unblock_withdraw(&self, address: String, comment: String) -> Result<String> {
        audit_account(address, comment, true,
                      false, true, false)
    }

    fn block_deposit(&self, address: String, comment: String) -> Result<String> {
        audit_account(address, comment, false,
                      false, false, true)
    }

    fn unblock_deposit(&self, address: String, comment: String) -> Result<String> {
        audit_account(address, comment, false,
                      true, false, true)
    }

    fn block(&self, address: String, comment: String) -> Result<String> {
        audit_account(address, comment, false,
                      false, true, true)
    }

    fn unblock(&self, address: String, comment: String) -> Result<String> {
        audit_account(address, comment, true,
                      true, true, true)
    }
}