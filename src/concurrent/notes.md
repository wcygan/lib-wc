# Concurrency

## Send + Sync

Send and Sync are traits that are used to determine whether a type is safe to share between threads. They are implemented automatically for types that are safe to share between threads. For example, `i32` is `Send` and `Sync` because it is safe to share between threads. `Rc<T>` is not `Send` because it is not safe to share between threads. `Arc<T>` is `Send` because it is safe to share between threads.

### PhantomData

We can make sure an object stays on the same thread by making sure its type does not implement Send, which can be
achieved with the PhantomData marker type

## Atomics

### `compare_exchange` vs. `compare_exchange_weak`

`compare_exchange` is a strong operation, which means that it will spin until the operation
succeeds. `compare_exchange_weak` is a weak operation, which means that it will not spin, but instead return `false` if
the operation fails.

The ARM processor architecture does not have a direct mapping for x86's `cmpxchg` instruction, so it will use a loop
of `LDREX` and `STREX` instructions to achieve the same behavior.

`compare_exchange_weak` does not attempt to spin, so it will use a single `LDREX` and `STREX` instruction. This is more
efficient, but it is also less reliable. If the operation fails, it is possible that the value has changed, so the
operation should be retried. 

