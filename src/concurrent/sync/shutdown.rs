use tokio::sync::broadcast;

/// Listens for a shutdown signal.
///
/// Shutdown is signalled using a `broadcast::Receiver`. Only a single value is
/// ever sent. Once a value has been sent via the broadcast channel, shutdown
/// should occur.
///
/// The `Shutdown` struct listens for the signal and tracks that the signal has
/// been received. Callers may query for whether the shutdown signal has been
/// received or not.
#[derive(Debug)]
pub struct Shutdown {
    /// `true` if the shutdown signal has been received
    shutdown: bool,

    /// The receive half of the channel used to listen for shutdown.
    notify: broadcast::Receiver<()>,
}

impl Shutdown {
    /// Create a new `Shutdown` backed by the given `broadcast::Receiver`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    /// use lib_wc::sync::Shutdown;
    ///
    /// let (tx, rx) = broadcast::channel(1);
    ///
    /// let shutdown = Shutdown::new(rx);
    /// ```
    pub fn new(notify: broadcast::Receiver<()>) -> Shutdown {
        Shutdown {
            shutdown: false,
            notify,
        }
    }

    /// Returns `true` if the shutdown signal has been received.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    /// use lib_wc::sync::Shutdown;
    ///
    /// let (tx, rx) = broadcast::channel(1);
    ///
    /// let shutdown = Shutdown::new(rx);
    ///
    /// assert!(!shutdown.is_shutdown());
    /// ```
    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    /// Receive the shutdown notice, waiting if necessary.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::task::spawn;
    /// use tokio::sync::broadcast;
    /// use lib_wc::sync::Shutdown;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///  let (tx, rx) = broadcast::channel(1);
    ///
    ///  let mut shutdown = Shutdown::new(rx);
    ///
    ///  assert!(!shutdown.is_shutdown());
    ///
    ///  let t = spawn(async move {
    ///     shutdown.recv().await;
    ///     assert!(shutdown.is_shutdown());
    ///  });
    ///
    ///  // Signal shutdown
    ///  drop(tx);
    ///
    ///  t.await;
    /// }
    /// ```
    pub async fn recv(&mut self) {
        // If the shutdown signal has already been received, then return
        // immediately.
        if self.shutdown {
            return;
        }

        // Cannot receive a "lag error" as only one value is ever sent.
        let _ = self.notify.recv().await;

        // Remember that the signal has been received.
        self.shutdown = true;
    }
}
