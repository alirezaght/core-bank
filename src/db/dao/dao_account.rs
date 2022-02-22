use diesel::{EqAll, ExpressionMethods, GroupByDsl, GroupedBy, QueryDsl, RunQueryDsl, sql_query};
use diesel::types::BigInt;
use uuid::Uuid;
use crate::db::dao::pagination::{Pagination};
use crate::db::establish_connection;
use crate::models::Account;
use crate::schema;
use crate::schema::t_account::{address, comment, created, deposit, detail, seq, withdraw};
use crate::schema::t_account::dsl::t_account;


pub fn find_by_address(account_address: String) -> Option<Account> {
    let connection = establish_connection();
    let result = t_account.filter(address.eq(account_address))
        .order(seq.desc()).first::<Account>(&connection);
    return result.ok();
}

pub fn save(account: Account) {
    let connection = establish_connection();
    let result = diesel::insert_into(t_account)
        .values(&account)
        .execute(&connection);
    if result.is_err() {
        println!("{:?}", result.err());
    }
}

pub fn list(page: i64, limit: i64) -> Pagination<Account> {
    let mut connection = establish_connection();
    let query = format!("select t.* from t_account t where seq=(select max(t2.seq) from t_account t2 where t2.address = t.address) limit {} offset {}", limit, page * limit);
    let count_query = "select count(1) from t_account t where seq=(select max(t2.seq) from t_account t2 where t2.address = t.address)";
    let result = sql_query(query)
        .load::<Account>(&mut connection);
    let count = diesel::expression::sql_literal::sql(count_query).get_result::<i64>(&connection);
    Pagination {
        data: result.unwrap(),
        page,
        limit,
        total: count.unwrap(),
    }
}