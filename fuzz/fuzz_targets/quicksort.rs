#![no_main]
extern crate lib_wc as wc;
use libfuzzer_sys::fuzz_target;
use wc::algorithms::sorting::*;

// this test tries to find inputs that break our quicksort implementation
fuzz_target!(|data: &[u8]| {
    quicksort(&mut data.to_vec());
});
