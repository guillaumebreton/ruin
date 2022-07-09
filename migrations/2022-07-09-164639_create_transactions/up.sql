create table transactions (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    description varchar not null,
    date_posted date not null,
    transaction_id varchar not null,
    transaction_amount integer not null,
    account_id integer not null,
    foreign key(account_id) references accounts(id)
)
