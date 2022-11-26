# Concurrency

## Atomics

### `compare_exchange` vs. `compare_exchange_weak`

`compare_exchange` is a strong operation, which means that it will spin until the operation succeeds. `compare_exchange_weak` is a weak operation, which means that it will not spin, but instead return `false` if the operation fails.

The ARM processor architecture does not have a direct mapping for x86's `cmpxchg` instruction, so it will use a loop of `LDREX` and `STREX` instructions to achieve the same behavior. 

`compare_exchange_weak` does not attempt to spin, so it will use a single `LDREX` and `STREX` instruction. This is more efficient, but it is also less reliable. If the operation fails, it is possible that the value has changed, so the operation should be retried. 