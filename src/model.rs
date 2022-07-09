use crate::schema::accounts;
use diesel::prelude::*; // important otherwise the filter function won't work
use diesel::{Queryable, SqliteConnection};

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub account_name: String,
    pub account_balance: i32,
    pub account_type: String,
    pub account_number: String,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub account_name: &'a str,
    pub account_balance: i32,
    pub account_type: &'a str,
    pub account_number: &'a str,
}

pub fn find_by_number(
    conn: &SqliteConnection,
    acc_number: &str,
) -> Result<Account, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;
    accounts
        .filter(account_number.eq(acc_number))
        .first::<Account>(conn)
}

pub fn upsert_account(
    conn: &SqliteConnection,
    acc_name: &str,
    acc_type: &str,
    acc_number: &str,
    acc_balance: i32,
) -> Result<Account, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;

    let r = find_by_number(conn, acc_number);

    match r {
        Ok(account) => Ok(account),
        Err(diesel::NotFound) => {
            println!("Creating a new account {}", acc_number);
            let new_account = NewAccount {
                account_name: acc_name,
                account_type: acc_type,
                account_number: acc_number,
                account_balance: acc_balance,
            };
            diesel::insert_into(accounts)
                .values(&new_account)
                .execute(conn)
                .expect("Error saving new account");
            find_by_number(conn, acc_number)
        }
        Err(e) => Err(e),
    }
}
