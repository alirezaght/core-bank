table! {
    use diesel::sql_types::*;
    use crate::models::*;

    t_transaction (id) {
        id -> Uuid,
        account -> Varchar,
        #[sql_name = "type"]
        type_ -> Transaction_type,
        seq -> Int8,
        amount -> Numeric,
        reason -> Transaction_reason,
        comment -> Nullable<Text>,
        description -> Nullable<Text>,
        balance -> Numeric,
        blocked -> Numeric,
        created -> Timestamptz,
    }
}
