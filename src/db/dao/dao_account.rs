use diesel::{EqAll, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;
use crate::db::establish_connection;
use crate::models::Account;
use crate::schema::t_account::{address, seq};
use crate::schema::t_account::dsl::t_account;


pub fn findByAddress(account_address: String) -> Option<Account> {
    let connection = establish_connection();
    let result = t_account.filter(address.eq(account_address))
        .order(seq.desc()).first::<Account>(&connection);
    return result.ok();
}





