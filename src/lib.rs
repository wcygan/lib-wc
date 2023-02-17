#![allow(dead_code)]

//! Will's Programming Toolbox
//!
//! # Concurrency Tools
//!
//! * [`executors::RayonThreadPool`], a thread pool which can wait for all tasks to complete before shutting down
//! * [`sync::ds::BasicSharedMap`], a concurrent map that can be cloned and shared between threads
//!
//! # Concurrency Primitives
//!
//! * [`sync::Mutex`], a primitive for mutual exclusion
//! * [`sync::SpinLock`], a primitive for mutual exclusion that spins in a loop
//! * [`sync::RwLock`], a primitive for mutual exclusion that allows multiple readers or one writer at a time
//! * [`sync::Semaphore`], a primitive to limit access
//! * [`sync::Condvar`], a primitive to signal and wait on a condition
//! * [`sync::oneshot::Channel`], a single-producer single-consumer channel that sends a single value
//! * [`sync::mpmc::Channel`], an unbounded multi-producer multi-consumer channel for message passing

pub use algorithms::sorting;
pub use concurrent::{executors, sync};

mod algorithms;
mod collections;
mod concurrent;
mod crates;
mod exercises;
mod language_features;
