use crate::schema::accounts;
use crate::schema::transactions;
use chrono::prelude::*;
use chrono::{Duration, Utc};
use diesel::prelude::*; // important otherwise the filter function won't work
use diesel::{Queryable, SqliteConnection};

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub account_name: String,
    pub account_balance: i32,
    pub account_number: String,
    pub account_type: String,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub account_name: &'a str,
    pub account_balance: i32,
    pub account_number: &'a str,
    pub account_type: &'a str,
}

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub date_posted: NaiveDate,
    pub transaction_id: String,
    pub transaction_amount: i32,
    pub account_id: i32,
    pub category_id: Option<i32>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub description: &'a str,
    pub date_posted: NaiveDate,
    pub transaction_id: &'a str,
    pub transaction_amount: i32,
    pub account_id: i32,
}
#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

pub struct NewCategory<'a> {
    pub name: &'a str,
}

pub struct Service<'a> {
    pub connection: &'a SqliteConnection,
}
impl Service<'_> {
    pub fn find_tx_by_tx_id(&self, tx_id: &str) -> Result<Transaction, diesel::result::Error> {
        use crate::schema::transactions::dsl::*;
        transactions
            .filter(transaction_id.eq(tx_id))
            .first::<Transaction>(self.connection)
    }

    pub fn upsert_transaction(
        &self,
        desc: &str,
        date: NaiveDate,
        tx_id: &str,
        amount: i32,
        acc_id: i32,
    ) -> Result<Transaction, diesel::result::Error> {
        use crate::schema::transactions::dsl::*;

        let r = self.find_tx_by_tx_id(tx_id);
        let new_tx = NewTransaction {
            transaction_id: tx_id,
            description: desc,
            transaction_amount: amount,
            account_id: acc_id,
            date_posted: date,
        };
        match r {
            Ok(tx) => {
                println!("Updating transaction {}", tx_id);
                diesel::update(transactions.filter(transaction_id.eq(tx_id)))
                    .set(&new_tx)
                    .execute(self.connection)
                    .unwrap();
                Ok(tx)
            }
            Err(diesel::NotFound) => {
                println!("Creating a new transaction {}", tx_id);
                diesel::insert_into(transactions)
                    .values(&new_tx)
                    .execute(self.connection)
                    .expect("Error saving new transaction");
                self.find_tx_by_tx_id(tx_id)
            }
            Err(e) => Err(e),
        }
    }

    pub fn find_account_by_number(
        &self,
        acc_number: &str,
    ) -> Result<Account, diesel::result::Error> {
        use crate::schema::accounts::dsl::*;
        accounts
            .filter(account_number.eq(acc_number))
            .first::<Account>(self.connection)
    }

    pub fn upsert_account(
        &self,
        acc_name: &str,
        acc_type: &str,
        acc_number: &str,
        acc_balance: i32,
    ) -> Result<Account, diesel::result::Error> {
        use crate::schema::accounts::dsl::*;

        let r = self.find_account_by_number(acc_number);

        match r {
            Ok(account) => {
                println!("Updating account {} balance to {}", acc_number, acc_balance);
                diesel::update(accounts.filter(account_number.eq(acc_number)))
                    .set(account_balance.eq(acc_balance))
                    .execute(self.connection)
                    .unwrap();
                Ok(account)
            }
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
                    .execute(self.connection)
                    .expect("Error saving new account");
                self.find_account_by_number(acc_number)
            }
            Err(e) => Err(e),
        }
    }

    pub fn list_transactions(
        &self,
        duration: Duration,
    ) -> Result<Vec<(Transaction, Account, Option<Category>)>, diesel::result::Error> {
        use crate::schema::accounts::dsl::*;
        use crate::schema::categories::dsl::*;
        use crate::schema::transactions::dsl::*;

        let txs = transactions
            .filter(date_posted.ge(Utc::today().naive_utc() - duration))
            .inner_join(accounts)
            .left_outer_join(categories)
            .order(date_posted.desc())
            .load::<(Transaction, Account, Option<Category>)>(self.connection)
            .unwrap();
        Ok(txs)
    }
}
