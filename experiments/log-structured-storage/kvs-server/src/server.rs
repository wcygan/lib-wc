use actix_broker::SystemBroker;
use crate::ServerResult;
use kvs_common::DEFAULT_ADDRESS;
use tokio::net::TcpListener;
use tracing::info;

pub type BrokerType = SystemBroker;

pub struct Listener {
    listener: TcpListener,
}

impl Listener {
    pub async fn new() -> ServerResult<Self> {
        let listener = TcpListener::bind(DEFAULT_ADDRESS).await?;
        Ok(Listener { listener })
    }

    pub async fn accept_connections(&mut self) -> ServerResult<()> {
        info!("accepting inbound connections");

        loop {
            let (socket, _addr) = self.listener.accept().await?;
            
        }

        Ok(())
    }
    
    
}
