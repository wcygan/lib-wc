#![allow(unused_macros)]

mod src;

extern crate lib_wc as wc;

#[macro_use]
extern crate criterion;

fn default_config() -> Criterion {
    Criterion::default()
        .sample_size(50)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3))
}

use criterion::{criterion_main, Criterion};
use std::time::Duration;
criterion_main!(src::concurrent::locks::bench::bench);
