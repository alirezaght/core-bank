use diesel::{EqAll, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;
use crate::db::establish_connection;
use crate::schema::t_transaction::dsl::t_transaction;
use crate::models::Transaction;
use crate::schema::t_transaction::{account, seq};


pub fn findById(id: Uuid) -> Option<Transaction> {
    let connection = establish_connection();
    let result = t_transaction.find(id).first::<Transaction>(&connection);
    return result.ok();
}

pub fn findByAccount(address: String) -> Option<Transaction> {
    let connection = establish_connection();
    let result = t_transaction.filter(account.eq(address))
        .order(seq.desc()).first::<Transaction>(&connection);
    return result.ok();
}



