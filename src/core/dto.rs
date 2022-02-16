use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDTO{
    id: String,
    comment: String,
    description: String,
    amount: BigDecimal,
    transaction_type: String,
    reason: String,
    balance: BigDecimal,
    blocked: BigDecimal,
    time: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDTO {
    address: String,
    withdraw: bool,
    deposit: bool,
    detail: String,
    comment: String,
    last_tx: TransactionDTO,
    time: chrono::DateTime<Utc>,
}
