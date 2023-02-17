#![allow(unused_macros)]
mod src;

#[macro_use]
extern crate criterion;
extern crate lib_wc as wc;

use criterion::{criterion_main, Criterion};
use std::time::Duration;

fn default_config() -> Criterion {
    Criterion::default()
        .sample_size(100)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(10))
}

criterion_main!(
    src::algorithms::sorting::bench,
    src::concurrent::sync::bench,
);
