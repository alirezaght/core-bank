table! {
    use diesel::sql_types::*;
    use crate::models::*;

    t_account (address, seq) {
        address -> Varchar,
        detail -> Nullable<Text>,
        seq -> Int8,
        withdraw -> Bool,
        deposit -> Bool,
        comment -> Nullable<Text>,
        created -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    t_transaction (id) {
        id -> Uuid,
        account -> Varchar,
        account_seq -> Int8,
        #[sql_name = "type"]
        type_ -> Transaction_type,
        seq -> Int8,
        amount -> Numeric,
        reason -> Transaction_reason,
        comment -> Nullable<Text>,
        description -> Nullable<Text>,
        balance -> Numeric,
        blocked -> Numeric,
        factor -> Nullable<Varchar>,
        created -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    t_account,
    t_transaction,
);
