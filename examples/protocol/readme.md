# PingPong

This examples demonstrates the use of the PingPong protocol.

A client and server will bounce a message back and forth until the counter is exhausted

## How to run

In one terminal run `cargo run --example server` and in another run `cargo run --example client`

### In the server
```bash
listening for connections on localhost:5050
new client: 127.0.0.1:56497
Ping(10)
Ping(8)
Ping(6)
Ping(4)
Ping(2)
Ping(0)
```

### In the client
```bash
Pong(9)
Pong(7)
Pong(5)
Pong(3)
Pong(1)

```