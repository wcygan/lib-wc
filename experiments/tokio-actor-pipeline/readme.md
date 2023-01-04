# Tokio Actor Pipeline

The `server` program demonstrates the usage of a pipeline of actors.

A pipeline is like an assembly line; each actor is responsible for one task & passes it to the next actor when finished. There will be a terminal actor who performs the final task in the pipeline.

## Usage

In order to see this project in action you need to run both the server and client.
The server can handle many connections from clients concurrently.

### Using the server

```sh
$ cargo run --bin server
```

### Using the client

Use `cargo run --bin client` to execute the binary; `ping` and `pong` are the only valid commands.

See the help information for the client:

```
$ cargo run --bin client
CLI to send requests to the server

Usage: client <COMMAND>

Commands:
  ping  Sends a ping to the server
  pong  Sends a pong to the server                               
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

### Architecture

The server is implemented as a system of actors built with [Tokio](https://github.com/tokio-rs/tokio).

There is an [accept loop](server/src/accept.rs) which listens for new connections &
starts to process them with the actors.

The server is composed of three steps:

1. [Reader](server/src/actors/reader.rs)
    - Purpose: network I/O
    - Read a request from the client
    - Pass the work to the next actor
2. [Processor](server/src/actors/processor.rs)
    - Purpose: server-side computation
    - Process the request
    - Pass the work to the next actor
3. [Responder](server/src/actors/responder.rs)
    - Purpose: network I/O
    - Respond to the client with the response

![pipeline](pipeline.png)