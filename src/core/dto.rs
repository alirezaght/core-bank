use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::dsl::t_transaction;
use crate::models::{Account, Transaction};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDTO{
    pub id: Uuid,
    pub comment: Option<String>,
    pub description: Option<String>,
    pub amount: BigDecimal,
    pub transaction_type: String,
    pub reason: String,
    pub balance: BigDecimal,
    pub blocked: BigDecimal,
    pub time: chrono::DateTime<Utc>,
}


impl From<Transaction> for TransactionDTO {
    fn from(transaction: Transaction) -> Self {
        Self {
            id: transaction.id,
            comment: transaction.comment,
            description: transaction.description,
            amount: transaction.amount,
            transaction_type: transaction.type_.to_string(),
            reason: transaction.reason.to_string(),
            balance: transaction.balance,
            blocked: transaction.blocked,
            time: transaction.created
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDTO {
    pub address: String,
    pub withdraw: bool,
    pub deposit: bool,
    pub detail: Option<String>,
    pub comment: Option<String>,
    pub last_tx: Option<TransactionDTO>,
    pub time: chrono::DateTime<Utc>,
}

impl From<Account> for AccountDTO {
    fn from(account: Account) -> Self {
        Self {
            address: account.address,
            withdraw: account.withdraw,
            deposit: account.deposit,
            detail: account.detail,
            comment: account.comment,
            time: account.created,
            last_tx: None
        }
    }
}
