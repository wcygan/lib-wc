use tokio::sync::{broadcast, mpsc};

/// A [`ShutdownController`] is used to signal that the application is shutting down and should wait
/// for all pending tasks to complete.
///
/// This is useful for things like web servers and database connections, etc where you want
/// to allow all in-flight processing to complete before shutting down in order to maintain a
/// consistent state.
///
/// Calling [`ShutdownController::shutdown`] will cause all [`ShutdownListener`] instances
/// to complete their [`ShutdownListener::recv`] calls.
pub struct ShutdownController {
    /// Used to tell all [`ShutdownListener`] instances that shutdown has started.
    notify_shutdown: broadcast::Sender<()>,

    /// Implicitly used to determine when all [`ShutdownListener`] instances have been dropped.
    task_tracker: mpsc::Sender<()>,

    /// Used to determine when all tasks have finished. Calling `recv()` on this channel
    /// will return when all of the send halves of the `task_tracker` channel have been dropped.
    task_waiter: mpsc::Receiver<()>,
}

impl ShutdownController {
    /// Create a new [`ShutdownController`].
    ///
    /// # Examples
    ///
    /// ```
    /// let shutdown = lib_wc::sync::ShutdownController::new();
    /// ```
    pub fn new() -> Self {
        let (notify_shutdown, _) = broadcast::channel::<()>(1);
        let (task_tracker, task_waiter) = mpsc::channel::<()>(1);

        Self {
            notify_shutdown,
            task_tracker,
            task_waiter,
        }
    }

    /// Create a new [`ShutdownListener`] instance that can listen for the shutdown signal.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_wc::sync::{ShutdownController, ShutdownListener};
    ///
    /// let shutdown = ShutdownController::new();
    /// let shutdown_listener = shutdown.subscribe();
    pub fn subscribe(&self) -> ShutdownListener {
        ShutdownListener::new(self.notify_shutdown.subscribe(), self.task_tracker.clone())
    }

    /// Begin shutting down and wait for all [`ShutdownListener`] instances to be dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use tokio::time::interval;
    /// use tokio::task::spawn;
    /// use tokio::sync::{broadcast, mpsc};
    /// use lib_wc::sync::{ShutdownController, ShutdownListener};
    ///
    /// async fn task(mut shutdown: ShutdownListener) {
    ///     let mut interval = interval(Duration::from_nanos(100));  
    ///     while !shutdown.is_shutdown() {
    ///        tokio::select! {
    ///           _ = interval.tick() => {
    ///             println!("tick");
    ///          }
    ///          _ = shutdown.recv() => {
    ///            println!("shutdown");
    ///            break;
    ///          }
    ///        }
    ///    }
    /// }   
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///  let shutdown = ShutdownController::new();
    ///
    ///  // Spawn a task
    ///  let t = spawn({
    ///   let shutdown_listener = shutdown.subscribe();
    ///   async move { task(shutdown_listener).await }
    ///  });
    ///
    /// // Wait for the task to finish
    /// shutdown.shutdown().await;
    /// }
    /// ```
    pub async fn shutdown(mut self) {
        // Notify all tasks that shutdown has started
        drop(self.notify_shutdown);

        // Destroy our mpsc::Sender so that the mpsc::Receiver::recv() will return immediately
        // once all tasks have completed (i.e. dropped their mpsc::Sender)
        drop(self.task_tracker);

        // Wait for all tasks to finish
        let _ = self.task_waiter.recv().await;
    }
}

/// Listens for a shutdown signal.
///
/// Shutdown is signalled using a [`broadcast::Receiver`]. Only a single value is
/// ever sent. Once a value has been sent via the broadcast channel, shutdown
/// should occur.
///
/// The [`ShutdownListener`] struct listens for the signal and tracks that the signal has
/// been received. Callers may query for whether the shutdown signal has been
/// received or not.
#[derive(Debug)]
pub struct ShutdownListener {
    /// `true` if the shutdown signal has been received
    shutdown_received: bool,

    /// The receive half of the channel used to listen for shutdown.
    shutdown_notifier: broadcast::Receiver<()>,

    /// Implicitly used to help [`ShutdownController`] understand when the program
    /// has completed shutdown.
    _task_tracker: mpsc::Sender<()>,
}

impl ShutdownListener {
    fn new(
        shutdown_notifier: broadcast::Receiver<()>,
        _task_tracker: mpsc::Sender<()>,
    ) -> ShutdownListener {
        ShutdownListener {
            shutdown_received: false,
            shutdown_notifier,
            _task_tracker,
        }
    }

    /// Returns `true` if the shutdown signal has been received.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_wc::sync::ShutdownController;
    /// use tokio::task::spawn;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let shutdown = ShutdownController::new();
    ///   let mut shutdown_listener = shutdown.subscribe();
    ///   assert!(!shutdown_listener.is_shutdown());
    ///
    ///   // Spawn a task
    ///   let t = spawn({
    ///    async move {
    ///     shutdown_listener.recv().await;
    ///     assert!(shutdown_listener.is_shutdown());
    ///    }
    ///   });
    ///
    ///   shutdown.shutdown().await;
    /// }
    /// ```
    pub fn is_shutdown(&self) -> bool {
        self.shutdown_received
    }

    /// Receive the shutdown notice, waiting if necessary.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::select;
    /// use lib_wc::sync::ShutdownListener;
    /// use lib_wc::sync::RateLimiter;
    ///
    /// /// A task that continuously does work at a fixed rate, but exits when shutdown is initiated.
    /// async fn long_lived_task(mut shutdown: ShutdownListener, rate_limiter: RateLimiter) {
    ///    while !shutdown.is_shutdown() {
    ///       select! {
    ///         _ = shutdown.recv() => { return; }
    ///         _ = rate_limiter.throttle(|| async { /* do work */ }) => { println!("tick"); }
    ///       }
    ///   }
    /// }
    /// ```
    pub async fn recv(&mut self) {
        // If the shutdown signal has already been received, then return
        // immediately.
        if self.shutdown_received {
            return;
        }

        // Cannot receive a "lag error" as only one value is ever sent.
        let _ = self.shutdown_notifier.recv().await;

        // Remember that the signal has been received.
        self.shutdown_received = true;
    }
}
