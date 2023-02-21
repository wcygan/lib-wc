# Client-Side Rate Limiting

> I made a [post](https://www.wcygan.io/post/tokio-client-side-rate-limiting/) about this topic

This example demonstrates how to use the `tokio` runtime to implement a rate limiting.

## Example

To run to example, you can use `cargo run`, which will expand to the following:

```
$ cargo run -- --period-ms 500 --clients 16 --radix 4
[Key 3 QPS] 1.9983383480914778 
[Key 1 QPS] 1.9982524946228102 
[Key 2 QPS] 1.998240382290497 
[Key 0 QPS] 1.9982806793035273 
[Key 2 QPS] 1.9992724314409915 
[Key 3 QPS] 1.9992347005093094 
[Key 0 QPS] 2.221336566941267 
[Key 1 QPS] 1.9991988765835114 
```