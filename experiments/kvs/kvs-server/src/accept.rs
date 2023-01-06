use crate::actors::System;
use kvs_common::connection::Connection;
use tokio::net::TcpListener;
use tokio::spawn;
use tracing::info;

/// Accepts connections and handles them in a loop
pub async fn accept_connections(listener: TcpListener) -> crate::Result<()> {
    let system = System::new()?;

    loop {
        let (stream, addr) = listener.accept().await?;

        spawn({
            info!("Accepted connection from {}", addr);
            let conn = Connection::new(stream);
            let mut system = system.clone();
            async move {
                system.handle_connection(conn).await;
            }
        });
    }
}
