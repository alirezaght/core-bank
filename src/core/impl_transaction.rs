use std::ops::{Add, Sub};
use bigdecimal::{BigDecimal, Zero};
use chrono::Utc;
use crate::core::address::Address;
use crate::core::dto::TransactionDTO;
use jsonrpc_derive::rpc;
use crate::db::dao::dao_transaction;
use crate::db::dao::dao_account;
use crate::db::dao::pagination::Pagination;
use crate::models::{Transaction, TransactionReason, TransactionType};
use jsonrpc_core::{Error, ErrorCode, Result};
use std::str::FromStr;
use serde_json::Value;
use uuid::Uuid;

pub struct CoreTransactionImpl;

#[rpc]
pub trait CoreTransaction {
    #[rpc(name = "list_transactions")]
    fn list_transactions(&self, address: String, page: i64, limit: i64) -> Result<Pagination<Transaction>>;
    #[rpc(name = "deposit")]
    fn deposit(&self, address: String, amount: BigDecimal, comment: String, description: String) -> Result<String>;
    #[rpc(name = "withdraw")]
    fn withdraw(&self, address: String, amount: BigDecimal, comment: String, description: String) -> Result<String>;
    #[rpc(name = "transfer")]
    fn transfer(&self, from: String, to: String, amount: BigDecimal, comment: String, description: String) -> Result<String>;
//     TODO: list transaction of all addresses
//     TODO: A crone job to revert the transfer transaction that had errors.
}

fn deposit(address: String, amount: BigDecimal, comment: String, description: String, reason: TransactionReason, factor: String) -> Result<String> {
    let mut transaction = dao_transaction::find_by_account(address.to_owned());
    let account = dao_account::find_by_address(address.to_owned());
    match account {
        None => Err(Error {
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None,
        }),
        Some(account) => {
            if !account.deposit {
                return Err(Error {
                    code: ErrorCode::from(-2),
                    message: "Couldn't deposit to account".to_string(),
                    data: None,
                });
            } else {
                if transaction.is_none() {
                    transaction = Option::from(Transaction::default());
                }
                let mut transaction_obj = transaction.unwrap().clone();
                transaction_obj.id = Uuid::new_v4();
                transaction_obj.account = account.address;
                transaction_obj.account_seq = account.seq;
                transaction_obj.seq += 1;
                transaction_obj.amount = amount;
                transaction_obj.balance = transaction_obj.balance.clone().add(transaction_obj.amount.clone());
                transaction_obj.reason = reason.to_owned();
                transaction_obj.type_ = TransactionType::DEPOSIT;
                transaction_obj.comment = Option::from(comment);
                transaction_obj.description = Option::from(description);
                if factor.is_empty() {
                    transaction_obj.factor = Option::from(transaction_obj.id.to_string());
                } else {
                    transaction_obj.factor = Option::from(factor);
                }
                transaction_obj.created = Utc::now();
                dao_transaction::save(transaction_obj);
                Ok("OK".to_string())
            }
        }
    }
}


fn withdraw(address: String, amount: BigDecimal, comment: String, description: String, reason: TransactionReason, factor: String) -> Result<String> {
    let mut transaction = dao_transaction::find_by_account(address.to_owned());
    let account = dao_account::find_by_address(address.to_owned());
    match account {
        None => Err(Error {
            code: ErrorCode::from(-1),
            message: "Couldn't find account".to_string(),
            data: None,
        }),
        Some(account) => {
            if !account.withdraw {
                return Err(Error {
                    code: ErrorCode::from(-2),
                    message: "Couldn't withdraw from account".to_string(),
                    data: None,
                });
            } else {
                match transaction {
                    None => Err(Error {
                        code: ErrorCode::from(-1),
                        message: "Couldn't find transaction".to_string(),
                        data: None,
                    }),
                    Some(mut transaction) => {
                        transaction.id = Uuid::new_v4();
                        transaction.seq += 1;
                        transaction.amount = amount;
                        transaction.balance = transaction.balance.clone().sub(transaction.amount.clone());
                        if transaction.balance.lt(&BigDecimal::zero()) {
                            return Err(Error {
                                code: ErrorCode::from(-3),
                                message: "Zero balance error".to_string(),
                                data: None,
                            });
                        }
                        if transaction.blocked.gt(&transaction.balance) {
                            return Err(Error {
                                code: ErrorCode::from(-4),
                                message: "blocked can't be greater than balance".to_string(),
                                data: None,
                            });
                        }
                        if factor.is_empty() {
                            transaction.factor = Option::from(transaction.id.to_string());
                        } else {
                            transaction.factor = Option::from(factor);
                        }
                        transaction.reason = reason;
                        transaction.type_ = TransactionType::WITHDRAW;
                        transaction.comment = Option::from(comment);
                        transaction.description = Option::from(description);
                        transaction.created = Utc::now();
                        dao_transaction::save(transaction);
                        Ok("OK".to_string())
                    }
                }
            }
        }
    }
}

impl CoreTransaction for CoreTransactionImpl {
    fn list_transactions(&self, address: String, page: i64, limit: i64) -> Result<Pagination<Transaction>> {
        Ok(dao_transaction::list_address(address, page, limit))
    }

    fn deposit(&self, address: String, amount: BigDecimal, comment: String, description: String) -> Result<String> {
        deposit(address, amount, comment, description, TransactionReason::DEPOSIT, "".to_string())
    }

    fn withdraw(&self, address: String, amount: BigDecimal, comment: String, description: String) -> Result<String> {
        withdraw(address, amount, comment, description, TransactionReason::WITHDRAW, "".to_string())
    }

    fn transfer(&self, from: String, to: String, amount: BigDecimal, comment: String, description: String) -> Result<String> {
        let from_account = dao_account::find_by_address(from.to_owned());
        match from_account {
            None => Err(Error {
                code: ErrorCode::from(-1),
                message: "Couldn't find account".to_string(),
                data: None,
            }),
            Some(from_account) => {
                if !from_account.withdraw {
                    return Err(Error {
                        code: ErrorCode::from(-2),
                        message: "Couldn't withdraw from account".to_string(),
                        data: None,
                    });
                } else {
                    let to_account = dao_account::find_by_address(to.to_owned());
                    match to_account {
                        None => Err(Error {
                            code: ErrorCode::from(-3),
                            message: "Couldn't find account".to_string(),
                            data: None,
                        }),
                        Some(to_account) => {
                            if !to_account.deposit {
                                return Err(Error {
                                    code: ErrorCode::from(-4),
                                    message: "Couldn't deposit to account".to_string(),
                                    data: None,
                                });
                            } else {
                                let factor = Uuid::new_v4();
                                let withdraw_result = withdraw(from, amount.to_owned(), comment.to_owned(), description.to_owned(), TransactionReason::TRANSFER, factor.to_owned().to_string());
                                match withdraw_result {
                                    Err(e) => Err(e),
                                    Ok(res) => {
                                        deposit(to, amount.to_owned(), comment.to_owned(), description.to_owned(), TransactionReason::TRANSFER, factor.to_owned().to_string())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}