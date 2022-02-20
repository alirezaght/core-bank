use chrono;
use bigdecimal;
use chrono::Utc;
use diesel_derive_enum::*;
use uuid::Uuid;
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::schema::t_transaction;
use crate::schema::t_account;

// TransactionType Enum

#[derive(Serialize, Deserialize, DbEnum)]
#[DieselType = "Transaction_type"]
#[derive(Debug)]
pub enum TransactionType {
    WITHDRAW,
    DEPOSIT,
    LOCK,
    UNLOCK,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// TransactionReason Enum

#[derive(Serialize, Deserialize, DbEnum)]
#[DieselType = "Transaction_reason"]
#[derive(Debug)]
pub enum TransactionReason {
    TRANSFER,
    REVERT,
    LOCK,
    UNLOCK,
    WITHDRAW,
    DEPOSIT,
}
impl fmt::Display for TransactionReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Transaction Entity

#[derive(Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[table_name = "t_transaction"]
#[derive(Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub account: String,
    pub account_seq: i64,
    pub type_: TransactionType,
    pub seq: i64,
    pub amount: bigdecimal::BigDecimal,
    pub reason: TransactionReason,
    pub comment: Option<String>,
    pub description: Option<String>,
    pub balance: bigdecimal::BigDecimal,
    pub blocked: bigdecimal::BigDecimal,
    pub created: chrono::DateTime<Utc>,
}

// Account Entity

#[derive(Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[table_name = "t_account"]
#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub detail: Option<String>,
    pub seq: i64,
    pub withdraw: bool,
    pub deposit: bool,
    pub comment: Option<String>,
    pub created: chrono::DateTime<Utc>,
}