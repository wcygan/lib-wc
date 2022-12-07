use std::mem::ManuallyDrop;
use std::ptr;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

use crossbeam::epoch::{pin, Atomic, Owned};

/// Treiber's lock-free stack.
///
/// Usable with any number of producers and consumers.
#[derive(Debug)]
pub struct TreiberStack<T> {
    head: Atomic<Node<T>>,
}

#[derive(Debug)]
struct Node<T> {
    data: ManuallyDrop<T>,
    next: Atomic<Node<T>>,
}

impl<T> TreiberStack<T> {
    /// Creates a new, empty stack.
    pub fn new() -> TreiberStack<T> {
        TreiberStack {
            head: Atomic::null(),
        }
    }

    /// Pushes a value on top of the stack.
    pub fn push(&self, t: T) {
        let mut n = Owned::new(Node {
            data: ManuallyDrop::new(t),
            next: Atomic::null(),
        });

        let guard = pin();

        loop {
            let head = self.head.load(Relaxed, &guard);
            n.next.store(head, Relaxed);

            match self
                .head
                .compare_exchange(head, n, Release, Relaxed, &guard)
            {
                Ok(_) => break,
                Err(e) => n = e.new,
            }
        }
    }

    /// Attempts to pop the top element from the stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn try_pop(&self) -> Option<T> {
        let guard = pin();
        loop {
            let head = self.head.load(Acquire, &guard);

            match unsafe { head.as_ref() } {
                Some(h) => {
                    let next = h.next.load(Relaxed, &guard);

                    if self
                        .head
                        .compare_exchange(head, next, Release, Relaxed, &guard)
                        .is_ok()
                    {
                        unsafe {
                            guard.defer_destroy(head);
                            return Some(ManuallyDrop::into_inner(ptr::read(&h.data)));
                        }
                    }
                }
                None => return None,
            }
        }
    }

    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        let guard = pin();
        self.head.load(Acquire, &guard).is_null()
    }
}

impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while self.try_pop().is_some() {}
    }
}

impl<T> Default for TreiberStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T> Send for TreiberStack<T> {}
unsafe impl<T> Sync for TreiberStack<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::spin_loop;
    use std::sync::Arc;
    use std::thread::scope;

    #[test]
    fn one_thread() {
        let s = TreiberStack::new();
        assert!(s.is_empty());

        for i in 0..100 {
            s.push(i);
        }

        for i in (0..100).rev() {
            assert_eq!(s.try_pop(), Some(i));
        }

        assert!(s.is_empty());
    }

    #[test]
    fn two_threads_pushing_one_pulling() {
        let stack = Arc::new(TreiberStack::<u32>::new());

        scope(|s| {
            // spawn 2 threads that push 1000 elements each
            for _ in 0..2 {
                let stack = stack.clone();
                s.spawn(move || {
                    for i in 0..1000 {
                        stack.push(i);
                    }
                });
            }

            // spawn 1 thread that pops 2000 elements
            s.spawn(|| {
                for _ in 0..2000 {
                    while stack.try_pop().is_none() {
                        spin_loop()
                    }
                }
            });
        })
    }
}
