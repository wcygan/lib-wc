# [lib-wc](https://crates.io/crates/lib-wc)

Learning how to write a library in Rust by implementing "stuff"

[![CI](https://github.com/wcygan/lib-wc/actions/workflows/general.yml/badge.svg)](https://github.com/wcygan/lib-wc/actions/workflows/general.yml)
[![Crates.io](https://img.shields.io/crates/v/lib-wc.svg)](https://crates.io/crates/lib-wc)

## Usage

### Run the tests

```bash
$ cargo test
```

### Run the benchmarks

```bash
$ cargo bench
```

### Run the fuzz tests

hint: you can use `cargo fuzz list` to see the available fuzz targets

```bash
$ cargo fuzz run <fuzz_target>
```