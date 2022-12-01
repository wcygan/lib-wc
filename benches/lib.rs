#![allow(unused_macros)]

mod src;

extern crate lib_wc as wc;

#[macro_use]
extern crate criterion;

use criterion::criterion_main;
criterion_main!(src::concurrent::locks::bench::bench);
