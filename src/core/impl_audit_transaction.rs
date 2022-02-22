use std::ops::{Add, Sub};
use bigdecimal::BigDecimal;
use chrono::Utc;
use jsonrpc_core::{Error, ErrorCode, Result};
use crate::core::address::Address;
use jsonrpc_derive::rpc;
use serde_json::Value;
use crate::dao_transaction;
use crate::models::{TransactionReason, TransactionType};
use std::str::FromStr;

pub struct CoreAuditTransactionImpl;

#[rpc]
pub trait CoreAuditTransaction {
    #[rpc(name = "lock")]
    fn lock(&self, address: String, amount: BigDecimal, comment: String, description: String)
            -> Result<String>;
    #[rpc(name = "unlock")]
    fn unlock(&self, address: String, amount: BigDecimal, comment: String, description: String)
              -> Result<String>;
}

fn audit_transaction(address: String, lock_amount: BigDecimal, unlock_amount: BigDecimal,
                     comment: String, description: String) -> Result<String> {
    let transaction = dao_transaction::find_by_account(address);
    match transaction {
        None => Err(Error {
            code: ErrorCode::from(-1),
            message: "Couldn't find transaction".to_string(),
            data: None,
        }),
        Some(mut transaction) => {
            let tmp_transaction = transaction.clone();
            let transaction_json = serde_json::to_string(&tmp_transaction).unwrap();
            let seq = transaction.seq;
            transaction.seq += 1;
            if lock_amount.gt(&BigDecimal::from_str(&"0").unwrap()) {
                transaction.reason = TransactionReason::LOCK;
                transaction.type_ = TransactionType::LOCK;
                let tmp_amount = lock_amount.clone();
                transaction.amount = lock_amount;
                transaction.blocked = transaction.blocked.add(tmp_amount);
            } else if unlock_amount.gt(&BigDecimal::from_str(&"0").unwrap()) {
                transaction.reason = TransactionReason::UNLOCK;
                transaction.type_ = TransactionType::UNLOCK;
                let tmp_amount = unlock_amount.clone();
                transaction.amount = unlock_amount;
                transaction.blocked = transaction.blocked.sub(tmp_amount);
            } else {
                return Err(Error {
                    code: ErrorCode::from(-2),
                    message: "Lock amount or unLock amount should be greater than zero".to_string(),
                    data: Option::from(Value::from(transaction_json.to_owned())),
                });
            }
            transaction.comment = Option::from(comment);
            transaction.description = Option::from(description);
            transaction.created = Utc::now();

            if transaction.blocked.gt(&transaction.amount) {
                return Err(Error {
                    code: ErrorCode::from(-2),
                    message: "Block amount is greater than balance".to_string(),
                    data: Option::from(Value::from(transaction_json.to_owned())),
                });
            } else if transaction.blocked.lt(&BigDecimal::from_str(&"0").unwrap()) {
                return Err(Error {
                    code: ErrorCode::from(-3),
                    message: "Block amount is lower than zero".to_string(),
                    data: Option::from(Value::from(transaction_json.to_owned())),
                });
            }

            dao_transaction::save(transaction);
            Ok("OK".to_string())
        }
    }
}

impl CoreAuditTransaction for CoreAuditTransactionImpl {
    fn lock(&self, address: String, amount: BigDecimal, comment: String, description: String)
            -> Result<String> {
        audit_transaction(address, amount, BigDecimal::from_str(&"0").unwrap(),
                          comment, description)
    }

    fn unlock(&self, address: String, amount: BigDecimal, comment: String, description: String)
              -> Result<String> {
        audit_transaction(address, BigDecimal::from_str(&"0").unwrap(), amount,
                          comment, description)
    }
}