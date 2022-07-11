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
    categories (id) {
        id -> Integer,
        name -> Text,
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
        category_id -> Nullable<Integer>,
    }
}

joinable!(transactions -> accounts (account_id));
joinable!(transactions -> categories (category_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    transactions,
);
