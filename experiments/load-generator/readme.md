## Load Generator

Adapted
from [A Load Test CLI with Async Rust](https://www.manning.com/liveprojectseries/a-load-test-cli-with-async-rust-ser)

Warning: you probably don't want to target a host on the internet or use a metered connection with this :)

### Usage

```
A tool to load test a server

Usage: load-generator-application [OPTIONS] --url <URL>

Options:
  -u, --url <URL>                  The URL to send requests to
  -c, --connections <CONNECTIONS>  The number of connections to use [default: 8]
  -t, --time <TIME>                The amount of seconds to run the test for. If not specified, the test will run until an interrupt signal is received
  -h, --help                       Print help

```

### Notes

Running [Axum Hello Server](../axum-hello-server) locally & aiming this tool at it yields around 1M - 2M QPS.

```
$ load-generator-application -u localhost:3000 -c 4
⠓ 1718637.41 requests per second                                        
```