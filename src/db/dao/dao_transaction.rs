use diesel::{EqAll, ExpressionMethods, QueryDsl, RunQueryDsl, sql_query};
use uuid::Uuid;
use crate::db::dao::pagination::Pagination;
use crate::db::establish_connection;
use crate::schema::t_transaction::dsl::t_transaction;
use crate::models::{Transaction};
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

pub fn save(transaction: Transaction) {
    let connection = establish_connection();
    diesel::insert_into(t_transaction)
        .values(&transaction)
        .execute(&connection);
}

pub fn list(page: i64, limit: i64) -> Pagination<Transaction> {
    let mut connection = establish_connection();
    let query = format!("select t.* from t_transaction t where seq=(select max(t2.seq) from t_transaction t2 where t2.account = t.account) limit {} offset {}", limit, page * limit);
    let count_query = "select count(1) from t_transaction t where seq=(select max(t2.seq) from t_transaction t2 where t2.account = t.account)";
    let result = sql_query(query)
        .load::<Transaction>(&mut connection);
    let count = diesel::expression::sql_literal::sql(count_query).get_result::<i64>(&connection);
    Pagination {
        data: result.unwrap(),
        page,
        limit,
        total: count.unwrap(),
    }
}

