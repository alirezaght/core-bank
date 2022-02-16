use bigdecimal::BigDecimal;
use crate::models::{TransactionReason, TransactionType};
use chrono::Utc;

pub struct TransactionDTO{
    id: String,
    comment: String,
    description: String,
    amount: BigDecimal,
    transaction_type: TransactionType,
    reason: TransactionReason,
    balance: BigDecimal,
    blocked: BigDecimal,
    time: chrono::DateTime<Utc>,
}

pub struct AccountDTO {
    address: String,
    withdraw: bool,
    deposit: bool,
    detail: String,
    comment: String,
    last_tx: TransactionDTO,
    time: chrono::DateTime<Utc>,
}
