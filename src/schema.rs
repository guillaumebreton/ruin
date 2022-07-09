table! {
    accounts (id) {
        id -> Integer,
        account_name -> Text,
        account_balance -> Integer,
        account_number -> Text,
        account_type -> Text,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        description -> Text,
        date_posted -> Date,
        transaction_id -> Text,
        transaction_amount -> Integer,
        account_id -> Integer,
    }
}

joinable!(transactions -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
