# [lib-wc](https://docs.rs/lib-wc/)

[<img alt="github" src="https://img.shields.io/badge/github-wcygan/lib--wc-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcygan/lib-wc)
[<img alt="crates.io" src="https://img.shields.io/crates/v/lib-wc.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/lib-wc)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lib--wc-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/lib-wc)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcygan/lib-wc/general.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcygan/lib-wc/actions?query=branch%3Amaster)

Learning how to write a library in Rust by implementing "stuff"

## Testing

### Run the tests

```bash
$ cargo test
```

or

```bash
$ cargo test --all-features
```

### Run the benchmarks

```bash
$ cargo bench --all-features
```

### Run the fuzz tests

hint: you can use `cargo fuzz list` to see the available fuzz targets

You need to use nightly Rust to run the fuzz tests:

```bash
$ cargo +nightly fuzz run <fuzz_target>
```

Example:

```bash
$ cargo +nightly fuzz run quicksort
```

## Addendum

I have a variety of [experiments](./experiments/) in this repository that I use to learn about Rust & various crates; they are runnable examples which showcase interesting concepts. 