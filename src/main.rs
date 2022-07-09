#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use clap::{Parser, Subcommand};
use diesel::prelude::*;

pub mod model;
pub mod ofx;
pub mod schema;

#[derive(Parser, Debug)]
#[clap(author = "Author Name", version, about)]
struct Arguments {
    // #[clap(default_value_t=usize::MAX,short, long)]
    // /// maximum depth to which sub-directories should be explored
    // max_depth: usize,
    // #[clap(short, long, parse(from_occurrences))]
    // verbosity: usize,
    #[clap(short, long, default_value_t = String::from("ruin.db"),forbid_empty_values = true)]
    /// the file to explore
    db_path: String,

    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Count how many times the package is used
    Import {
        #[clap(short, long, default_value_t = String::from("data.ofx"),forbid_empty_values = true)]
        /// the file to explore
        file_path: String,
    },
}

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

fn main() {
    let args = Arguments::parse();
    let connection = SqliteConnection::establish(&args.db_path)
        .unwrap_or_else(|_| panic!("Error connecting to db"));
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();
    match args.cmd {
        SubCommand::Import { file_path } => {
            let data = ofx::load(&file_path).unwrap();
            let account = data.message.response.aggregate.account;
            model::upsert_account(
                &connection,
                "",
                &account.account_type,
                &account.account_number,
                0,
            )
            .unwrap();
        }
    }
}
