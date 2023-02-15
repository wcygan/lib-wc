use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio::time::sleep;
use tokio::{select, spawn};

static MAXIMUM_CONNECTIONS: usize = 5;
static TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<()> {
    let sem = Arc::new(Semaphore::new(MAXIMUM_CONNECTIONS));
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop_until_shutdown(&mut listener, &sem).await
}

/// [`loop_until_shutdown`] is used to accept new connections & cause a timeout
/// when the server is idle for a while. The timeout is reset
/// every time a new connection is accepted.
async fn loop_until_shutdown(listener: &mut TcpListener, sem: &Arc<Semaphore>) -> Result<()> {
    loop {
        let next_conn_permit = sem.clone().acquire_owned().await?;

        let conn: TcpStream = select! {
            conn = listener.accept() => conn?.0,
            _ = timeout(sem, TIMEOUT) => return Ok(()),
        };

        spawn(handle(conn, next_conn_permit));
    }
}

/// [`timeout`] is used to cause a shutdown when the server is idle.
///
/// It works by acquiring all the permits except one, then sleeping.
/// This works because it allows the server to accept one more connection,
/// which will reset the timeout.
///
/// Additionally, when a new connection is accepted, the future returned
/// by this function will be dropped, which will release all of the permits
/// acquired by this function.
async fn timeout(sem: &Semaphore, timeout: Duration) -> Result<()> {
    let _permits = sem.acquire_many(MAXIMUM_CONNECTIONS as u32 - 1).await?;
    sleep(timeout).await;
    Ok(())
}

/// [`handle`] is used to handle a connection.
///
/// For the purposes of this example, it does not do anything useful.
///
/// In the real world this is where you would begin processing the connection.
async fn handle(_conn: TcpStream, _next_conn_permit: OwnedSemaphorePermit) -> Result<()> {
    sleep(TIMEOUT).await;
    Ok(())
}
