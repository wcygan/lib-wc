
## Publishing to [crates.io](https://crates.io/)

```zsh
$ cargo publish
```

## Development Loop

```zsh
$ cargo install cargo-watch
$ cargo watch -x check -x test
```

This will run `cargo check`. If that passes, it will run `cargo test`. This happens every time after a file changes.

## Formatting

rustfmt and clippy are used to format and lint the code. This helps us write idiomatic Rust code.

```zsh
$ cargo fmt
$ cargo clippy
```

## Benchmark testing

TODO: investigate [criterion.rs](https://github.com/bheisler/criterion.rs) to benchmark the library