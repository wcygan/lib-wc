use tracing::info;

/// Potentially use the actor model?
///  1. Listener    - accepts requestes   - (networking I/O)
///  2. Processor   - reads/writes to db  - (file I/O)
///  3. Responder   - responds to clients - (networking I/O)
fn main() -> Result<(), ServerError> {
    tracing_subscriber::fmt::try_init().map_err(|_| ServerError::TracingInitializationError)?;
    info!("server started");
    Ok(())
}

#[derive(Debug)]
enum ServerError {
    TracingInitializationError,
}
