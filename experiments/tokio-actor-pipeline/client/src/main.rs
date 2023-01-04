use clap::{Parser, Subcommand};
use tokio::net::TcpStream;

use common::connection::Connection;
use common::messages::{Request, Response};
use common::ADDRESS;

#[tokio::main(flavor = "current_thread")]
async fn main() -> common::Result<()> {
    let cli = Cli::parse();
    let stream = TcpStream::connect(ADDRESS).await?;
    let mut connection = Connection::new(stream);

    let request = match cli.command {
        Command::Ping => Request::ping(),
        Command::Pong => Request::pong(),
    };

    send_request(&mut connection, request).await?;
    match read_response(&mut connection).await? {
        None => {
            println!("no response");
        }
        Some(resp) => {
            println!("Received response: {:?}", resp);
        }
    }
    Ok(())
}

async fn send_request(connection: &mut Connection, request: Request) -> common::Result<()> {
    connection.write(&request).await?;
    Ok(())
}

async fn read_response(connection: &mut Connection) -> common::Result<Option<Response>> {
    let response = connection.read::<Response>().await?;
    Ok(response)
}

#[derive(Debug, Parser)]
#[command(name = "cli")]
#[command(about = "CLI to send requests to the server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Sends a ping to the server
    #[command(arg_required_else_help = false)]
    Ping,
    /// Sends a pong to the server
    #[command(arg_required_else_help = false)]
    Pong,
}
