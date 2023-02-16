# Tokio Idle Shutdown

This example shows how to shut down a Tokio application when it is idle.

The application will shut down when both conditions are met:

1. There are no active connections
2. A timeout has elapsed since the last connection was closed

Credit: this example was inspired
by [a conversation on the Tokio discord](https://discord.com/channels/500028886025895936/500336333500448798/1074602723959377980)

## Design

Idle shutdown is achieved by using a semaphore to track the number of active connections.

When a new connection is accepted, a permit is acquired from the semaphore. When the connection closes, the permit is
returned to the semaphore.

The server runs a timeout task which will shut down the server if there are no active connections & the timeout elapses.
This timeout is reset whenever a new connection is accepted.

## Problems & Insights

Here are some things that I learned while writing this example.

### Keeping the server alive until all connections are closed

Consider this code snippet:

```rust
    loop {
        let next_conn_permit = sem.clone().acquire_owned().await?;

        let conn: TcpStream = select! {
            conn = listener.accept() => conn?.0,
            _ = timeout(sem, TIMEOUT) => ...
        };
        ...
    }
```

By acquiring an owned permit before entering `select!`, we ensure that the server will
NOT shut down until all active connections have been closed.

This is because there will be two owned permits that exist, hence `timeout(...)` will not
be able to acquire N-1 permits, where N is maximum number of connections.

### Returning permits to the semaphore (and accepting concurrent connections)

Consider this code snippet:

```rust
async fn timeout(sem: &Semaphore, timeout: Duration) -> Result<()> {
    let _permits = sem.acquire_many(MAXIMUM_CONNECTIONS as u32 - 1).await?;
    sleep(timeout).await;
    Ok(())
}
```

The function returns a future which completes once it has acquired N-1 semaphore permits & sleeps for some time.

The permits are returned to the semaphore when the future is dropped & the future is dropped if a new connection is
accepted

By dropping the future, permits are returned to the semaphore, allowing more than one connection to be
handled at a time.

### Futures can be dropped

This might seem obvious, but it didn't truly click until I wrote this example.

This example works only because the `timeout` future is dropped when a new connection is accepted. If the future wasn't
dropped we would encounter deadlock.