use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::*;

/// This is a great example of how to _not_ implement a channel properly.
/// Don't use it! It's bad!
const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}
unsafe impl<T: Send> Sync for Channel<T> {}
impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    /// Send a message to the channel
    ///
    /// # Panics
    ///
    /// Panics if a message was already sent
    pub fn send(&self, message: T) {
        if self
            .state
            .compare_exchange(EMPTY, WRITING, Relaxed, Relaxed)
            .is_err()
        {
            panic!("can't send more than one message!");
        }
        unsafe { (*self.message.get()).write(message) };
        self.state.store(READY, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Relaxed) == READY
    }

    /// Receive a message from the channel
    ///
    /// # Panics
    ///
    /// Panics if no message is available
    pub fn receive(&self) -> T {
        if self
            .state
            .compare_exchange(READY, READING, Acquire, Relaxed)
            .is_err()
        {
            panic!("no message available!");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::scope;

    #[test]
    fn test_send_receive() {
        let channel = Channel::<u32>::new();
        let message = 42;

        scope(|s| {
            s.spawn(|| {
                channel.send(message);
            });

            s.spawn(|| {
                while !channel.is_ready() {}
                assert_eq!(channel.receive(), message);
            });
        })
    }

    #[test]
    #[should_panic]
    fn empty_receive_should_panic() {
        let channel = Channel::<u32>::new();

        // panics
        channel.receive();
    }

    #[test]
    #[should_panic]
    fn second_send_should_panic() {
        let channel = Channel::<u32>::new();
        channel.send(1);

        // panics
        channel.send(2);
    }
}
