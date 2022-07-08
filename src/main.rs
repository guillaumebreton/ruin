use clap::{Parser, Subcommand};
mod ofx;

#[derive(Parser, Debug)]
#[clap(author = "Author Name", version, about)]

struct Arguments {
    // #[clap(default_value_t=usize::MAX,short, long)]
    // /// maximum depth to which sub-directories should be explored
    // max_depth: usize,
    // #[clap(short, long, parse(from_occurrences))]
    // verbosity: usize,
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

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Import { file_path } => {
            ofx::Load(&file_path).unwrap();
        }
    }
}
