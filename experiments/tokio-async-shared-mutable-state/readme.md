# Shared Mutable State in (Asynchronous) Rust

This is a concrete example of the topics in the
post [Shared mutable state in Rust](https://draft.ryhl.io/blog/shared-mutable-state/).

### Takeaways

You should use a wrapper struct to hold the shared state because

- It can hide `lock` calls, making code easier to read
- It avoids cluttered function signatures
- You *don't* leak implementation details
- You limit the lifetime of the `MutexGuard`, holding onto it only as long as you need it

```rust

```rust
#[derive(Clone)]
pub struct SharedMap {
    inner: Arc<Mutex<SharedMapInner>>,
}

struct SharedMapInner {
    data: HashMap<i32, String>,
}
```

---

You cannot `.await` anything while a mutex is locked

---


