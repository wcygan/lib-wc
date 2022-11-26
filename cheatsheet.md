
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