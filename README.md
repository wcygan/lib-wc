# lib-wc

idk yet lol

[![CI](https://github.com/wcygan/lib-wc/actions/workflows/general.yml/badge.svg)](https://github.com/wcygan/lib-wc/actions/workflows/general.yml)
[![Crates.io](https://img.shields.io/crates/v/lib-wc.svg)](https://crates.io/crates/lib-wc)

## Development Loop

While hacking on this library, you can speed up the perceived time to iterate by watching the repository:

```zsh
$ cargo install cargo-watch
$ cargo watch -x check -x test
```

This will run `cargo check`. If that passes, it will run `cargo test`. This happens every time after a file changes.