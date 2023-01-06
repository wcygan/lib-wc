use clap::Subcommand;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Subcommand)]
pub enum Request {
    /// Retrieves the value of a key-value pair
    #[command(arg_required_else_help = true)]
    Get { key: String },
    /// Inserts a key-value pair
    #[command(arg_required_else_help = true)]
    Put { key: String, value: String },
    /// Updates a key-value pair
    #[command(arg_required_else_help = true)]
    Update { key: String, value: String },
    /// Deletes a key-value pair
    #[command(arg_required_else_help = true)]
    Delete { key: String },
}

/// A response from the server.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    KeyNotFound,
    Ok,
    OkWithValue { value: String },
}
