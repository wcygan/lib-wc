# Client-Side Rate Limiting

This example demonstrates how to use the `tokio` runtime to implement a "best-effort" rate limited client.

## Design

The design is simple, we use `tokio::sync::Mutex` and `tokio::time::interval` in tandem.

The interval is designed to be equal to approximately (1 / QPS) seconds. We then use a `Mutex` to guard the interval, and only allow the interval to tick when the mutex is locked. This ensures that we only send a request when the interval has ticked.

The `Mutex` ensures that the actual QPS will hover around the maximum QPS by allowing approximately one request per tick of the interval. However, it does not guarantee that we will not exceed the maximum QPS. 

## Example

Let's run an example with 5000 max QPS and 100 concurrent clients sending requests:

```
$ cargo run -- --qps 2.25 --concurrency 50
QPS: 2.2406448841268873
QPS: 3.1711702569599263
QPS: 2.7664498208719936
QPS: 2.569741994691556
QPS: 2.4529657021523623
QPS: 2.37704602113907
QPS: 2.321471075594861
QPS: 2.2807433749738095
QPS: 2.4741771118879967
QPS: 2.426148686515379
QPS: 2.3873097956830094
```

And with a higher level of concurrency:

```
$ cargo run -- --qps 1000 --concurrency 500 
QPS: 700.4027315706531
QPS: 1001.60545606153
QPS: 999.0287913161344
QPS: 999.4357652782361
QPS: 999.778166603592
QPS: 999.7976528631222
QPS: 999.633326101864
QPS: 999.774358464143
QPS: 999.859725077116
QPS: 999.779181186737
QPS: 999.8395815159078
QPS: 999.8632880493105

```

We see that the QPS is not exactly matching since we print out timings, but it is close enough.