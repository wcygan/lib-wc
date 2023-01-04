use tokio::net::TcpListener;
use tokio::spawn;
use tracing::info;

use common::connection::Connection;

use crate::actors::ActorSystem;

/// Accepts connections and handles them in a loop
pub async fn accept_connections(listener: TcpListener, system: ActorSystem) -> std::io::Result<()> {
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);
        let conn = Connection::new(stream);

        spawn({
            let mut system = system.clone();
            async move {
                system.handle(conn).await;
            }
        });
    }
}
