# Client-Side Rate Limiting

This example demonstrates how to use the `tokio` runtime to implement a "best-effort" rate limited client.

## Design

The design is simple, we use `tokio::sync::Mutex` and `tokio::time::interval` in tandem.

The interval is designed to be equal to approximately (1 / QPS) seconds. We then use a `Mutex` to guard the interval, and only allow the interval to tick when the mutex is locked. This ensures that we only send a request when the interval has ticked.

The `Mutex` ensures that the actual QPS will hover around the maximum QPS by allowing approximately one request per tick of the interval. However, it does not guarantee that we will not exceed the maximum QPS. 

## Example

Let's run an example with 5000 max QPS and 100 concurrent clients sending requests:

```
$ cargo run -- --qps 5000 --concurrency 100
QPS: 4974.852716975981
QPS: 5000.153129689596
QPS: 5001.982493061274
QPS: 4998.631287142516
QPS: 4998.31187062846
QPS: 5000.819883369496
QPS: 5000.509733896819
QPS: 5001.585605649052
```

We see that the QPS is not exactly 5000, but it is close enough.