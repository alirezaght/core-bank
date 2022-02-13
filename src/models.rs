use uuid;
use chrono;
use bigdecimal;
use chrono::Utc;
use diesel_derive_enum::*;

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
pub struct Transaction {
    pub id: uuid::Uuid,
    pub transaction_type: TransactionType,
    pub amount: String,
    pub balance: bigdecimal::BigDecimal,
    pub blocked: bigdecimal::BigDecimal,
    pub seq: i64,
    pub reason: TransactionReason,
    pub comment: Option<String>,
    pub description: Option<String>,
    pub created: chrono::DateTime<Utc>,
}