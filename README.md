# lib-wc

IDK yet!

## Development Loop

While hacking on this library, you can speed up the perceived time to iterate by watching the repository:

```zsh
$ cargo install cargo-watch
$ cargo watch -x check -x test
```

This will run `cargo check`. If that passes, it will run `cargo test`. This happens every time after a file changes.