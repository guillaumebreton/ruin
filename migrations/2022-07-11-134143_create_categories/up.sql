-- Your SQL goes here
create table categories (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    name varchar not null
);


-- This part is due to an interesting property of SQLite. Because The engine stores the structure of the table as a raw
-- create statement in the table itself, sqlite doesn't support adding contraints using alter table. Therefore, you need
-- to create a new table, copy the data and drop the existing one, to create a new structure.
-- NOTE: We don't start a transaction because we already use a transaction with diesel
PRAGMA foreign_keys=off;

create table new_transactions (
    id integer PRIMARY KEY AUTOINCREMENT not null,
    description varchar not null,
    date_posted date not null,
    transaction_id varchar not null,
    transaction_amount integer not null,
    account_id integer not null,
    category_id integer,
    foreign key(account_id) references accounts(id),
    foreign key(category_id) references categories(id)
);

INSERT INTO new_transactions(id, description, date_posted, transaction_id, transaction_amount, account_id)
SELECT id, description, date_posted, transaction_id, transaction_amount, account_id
FROM transactions;

drop table transactions;

alter table new_transactions rename to transactions;

PRAGMA foreign_keys=on;

