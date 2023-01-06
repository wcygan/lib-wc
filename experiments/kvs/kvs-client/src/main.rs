use clap::Parser;
use kvs_common::connection::Connection;
use kvs_common::requests::{Request, Response};
use kvs_common::DEFAULT_ADDRESS;

#[tokio::main(flavor = "current_thread")]
async fn main() -> kvs_common::Result<()> {
    let cli = Cli::parse();
    let mut conn = Connection::dial(DEFAULT_ADDRESS).await?;
    conn.write::<Request>(&cli.command.into()).await?;
    let response = conn.read::<Response>().await?;
    match response {
        Some(response) => {
            println!("{:?}", response);
        }
        None => println!("rip, no response"),
    }
    Ok(())
}

/// A client to interact with the key-value store server
#[derive(Debug, Parser)]
#[command(name = "kvs-client")]
#[command(about = "A client to interact with the key-value store server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Request,
}
