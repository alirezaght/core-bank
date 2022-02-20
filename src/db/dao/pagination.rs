use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub limit: i64,
    pub total: i64
}