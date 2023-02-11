use std::str::FromStr;

use crate::updates::{Update, REQUESTS_PER_UPDATE};
use anyhow::{Context, Result};
use hyper::Uri;
use tokio::io::AsyncWriteExt;
use tokio::select;
use tokio::sync::mpsc;

use crate::shutdown::Shutdown;

pub struct Connection {
    client: Client,
    updater: mpsc::Sender<Update>,
    shutdown: Shutdown,
    _shutdown_complete: mpsc::Sender<()>,
}

impl Connection {
    pub async fn new(
        uri: String,
        updater: mpsc::Sender<Update>,
        shutdown: Shutdown,
        shutdown_complete: mpsc::Sender<()>,
    ) -> Result<Self> {
        Ok(Self {
            client: Client::new(uri).await?,
            updater,
            shutdown,
            _shutdown_complete: shutdown_complete,
        })
    }

    pub async fn run(&mut self) {
        let mut request_count = 0;
        while !self.shutdown.is_shutdown() {
            select! {
                _ = self.shutdown.recv() => { return }
                _ = self.client.send_request() => {
                    request_count += 1;
                    if request_count % REQUESTS_PER_UPDATE == 0 {
                        self.updater.send(()).await.unwrap();
                    }
                }
            }
        }
    }
}

struct Client {
    stream: tokio::net::TcpStream,
    request: String,
}

impl Client {
    pub async fn new(uri: String) -> Result<Self> {
        let uri = Uri::from_str(&uri)?;

        let addr = match uri.authority() {
            None => {
                return Err(anyhow::anyhow!("No authority in {uri}"));
            }
            Some(auth) => auth.as_str(),
        };

        let stream = tokio::net::TcpStream::connect(addr).await?;

        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", uri.path(), addr);

        Ok(Self { stream, request })
    }

    async fn send_request(&mut self) -> Result<()> {
        self.stream
            .write_all(self.request.as_bytes())
            .await
            .context("error writing request")
    }
}
