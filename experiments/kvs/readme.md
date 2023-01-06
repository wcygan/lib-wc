# kvs

A client-server wrapper around the key-value store found in the
book [Rust in Action](https://livebook.manning.com/book/rust-in-action/chapter-7?origin=product-toc).

The [KVStore](kvs/src/lib.rs) implements a key-value store
using [log-structured storage](https://en.wikipedia.org/wiki/Log-structured_file_system).