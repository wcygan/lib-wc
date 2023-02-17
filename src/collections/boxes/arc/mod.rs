use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::atomic::{fence, AtomicUsize};

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

struct ArcData<T> {
    data_ref_count: AtomicUsize,
    alloc_ref_count: AtomicUsize,
    data: UnsafeCell<ManuallyDrop<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}

unsafe impl<T: Send + Sync> Sync for Arc<T> {}

unsafe impl<T: Send + Sync> Sync for Weak<T> {}

unsafe impl<T: Send + Sync> Send for Weak<T> {}

impl<T> Arc<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn new(data: T) -> Self {
        Self {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                data_ref_count: AtomicUsize::new(1),
                alloc_ref_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // Acquire matches Weak::drop's Release decrement, to make sure any
        // upgraded pointers are visible in the next data_ref_count.load.
        if arc
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Acquire, Relaxed)
            .is_err()
        {
            return None;
        }

        let is_unique = arc.data().data_ref_count.load(Relaxed) == 1;
        // Release matches Acquire increment in `downgrade`, to make sure any
        // changes to the data_ref_count that come after `downgrade` don't
        // change the is_unique result above.
        arc.data().alloc_ref_count.store(1, Release);
        if !is_unique {
            return None;
        }

        // Acquire to match Arc::drop's Release decrement, to make sure nothing
        // else is accessing the data.
        fence(Acquire);
        unsafe { Some(&mut *arc.data().data.get()) }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        let mut ref_count = arc.data().alloc_ref_count.load(Relaxed);
        loop {
            if ref_count == usize::MAX {
                std::hint::spin_loop();
                ref_count = arc.data().alloc_ref_count.load(Relaxed);
                continue;
            }
            assert!(ref_count < usize::MAX - 1);

            // Acquire synchronises with get_mut's release-store.
            if let Err(e) = arc.data().alloc_ref_count.compare_exchange_weak(
                ref_count,
                ref_count + 1,
                Acquire,
                Relaxed,
            ) {
                ref_count = e;
                continue;
            }
            return Weak { ptr: arc.ptr };
        }
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }

            assert!(n < usize::MAX);

            if let Err(e) =
                self.data()
                    .data_ref_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            };

            return Some(Arc { ptr: self.ptr });
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: Since there's an Arc to the data,
        // the data exists and may be shared.
        unsafe { &*self.data().data.get() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort()
        }

        Self { ptr: self.ptr }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort()
        }

        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe { ManuallyDrop::drop(&mut *self.data().data.get()) }

            // Now that there's no `Arc<T>`s left,
            // drop the implicit weak pointer that represented all `Arc<T>`s.
            drop(Weak { ptr: self.ptr });
        }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe { drop(Box::from_raw(self.ptr.as_ptr())) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::spawn;

    #[test]
    fn test() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;
        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        let x = Arc::new(("hello", DetectDrop));
        let y = x.clone();

        let t = spawn(move || {
            assert_eq!(x.0, "hello");
        });

        // x and y should be usable in parallel
        assert_eq!(y.0, "hello");

        t.join().unwrap();

        // x has been dropped, but y hasn't, so the arc and its data should still exist
        assert_eq!(NUM_DROPS.load(Relaxed), 0);

        drop(y);
        assert_eq!(NUM_DROPS.load(Relaxed), 1);
    }

    #[test]
    fn test_get_mut() {
        let mut x = Arc::new(1);
        assert_eq!(Arc::get_mut(&mut x), Some(&mut 1));
        let y = x.clone();
        assert_eq!(Arc::get_mut(&mut x), None);
        drop(y);
        assert_eq!(Arc::get_mut(&mut x), Some(&mut 1));
    }

    #[test]
    fn test_upgrade_downgrade() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);
        struct DetectDrop;
        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        // Create an Arc with two weak pointers.
        let x = Arc::new(("hello", DetectDrop));
        let y = Arc::downgrade(&x);
        let z = Arc::downgrade(&x);

        let t = spawn(move || {
            let y = y.upgrade().unwrap();
            assert_eq!("hello", y.0)
        });

        assert_eq!("hello", x.0);
        t.join().unwrap();

        // the data shouldn't be dropped yet
        // and the weak pointer should be upgradable
        assert_eq!(0, NUM_DROPS.load(Relaxed));
        assert!(z.upgrade().is_some());

        // drop the only "strong" Arc
        drop(x);

        // the data should be dropped
        // the weak pointer should no longer be upgradable
        assert_eq!(1, NUM_DROPS.load(Relaxed));
        assert!(z.upgrade().is_none());
    }
}
