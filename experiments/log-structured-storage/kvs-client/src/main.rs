use clap::{Parser, Subcommand};

// todo: benchmark with wrk (https://github.com/wg/wrk)
fn main() {
    let _cli = Cli::parse();
    println!("{:?}", _cli);
}

/// A client to interact with the key-value store server
#[derive(Debug, Parser)]
#[command(name = "kvs-client")]
#[command(about = "A client to interact with the key-value store server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Retrieves the value of a key-value pair
    #[command(arg_required_else_help = true)]
    Get {
        /// The key to search for
        key: String,
    },
    /// Inserts a key-value pair
    #[command(arg_required_else_help = true)]
    Insert {
        /// The key to insert
        key: String,
        /// The value to insert
        value: String,
    },
    /// Deletes a key-value pair
    #[command(arg_required_else_help = true)]
    Delete {
        /// The key to delete
        key: String,
    },
    /// Updates a key-value pair
    #[command(arg_required_else_help = true)]
    Update {
        /// The key to update
        key: String,
        /// The value to update
        value: String,
    },
}
