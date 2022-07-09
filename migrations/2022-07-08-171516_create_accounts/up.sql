create table accounts (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    account_name varchar not null,
    account_balance integer default 0 not null,
    account_number varchar not null unique,
    account_type varchar not null
)
