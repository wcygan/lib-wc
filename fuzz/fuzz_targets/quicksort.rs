#![no_main]
extern crate lib_wc as wc;
use libfuzzer_sys::fuzz_target;
use wc::sorting::*;

fuzz_target!(|data: &[u8]| {
    QuickSort::sort(&mut data.to_vec());
});
