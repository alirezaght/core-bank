use chrono;
use bigdecimal;
use chrono::Utc;
use diesel_derive_enum::*;
use uuid::Uuid;

// TransactionType Enum

#[derive(DbEnum)]
#[DieselType = "Transaction_type"]
#[derive(Debug)]
pub enum TransactionType {
    WITHDRAW,
    DEPOSIT,
    LOCK,
    UNLOCK,
}

// TransactionReason Enum

#[derive(DbEnum)]
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

// Transaction Entity

#[derive(Queryable)]
#[derive(Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub account: String,
    pub account_seq: i64,
    pub transaction_type: TransactionType,
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

#[derive(Queryable)]
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