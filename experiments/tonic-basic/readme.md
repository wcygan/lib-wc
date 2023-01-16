# Tonic Basic Example

Tonic is a gRPC client and server implementation for Rust.

This crate is following the [tonic hello world example](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md).Å

## Running the server

```bash
cargo run --bin server
```

## Running the client
    
```bash
cargo run --bin client
```

## Reaching the server via grpcurl

```bash
$ grpcurl -plaintext -import-path ./proto -proto proto/hello.proto -d '{"name": "Tonic"}' '[::]:50051' hello.Greeter/SayHello
{
  "message": "Hello Tonic!"
}
