use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::Arc;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel: Arc<Channel<T>> = Arc::new(Channel::new());
    (
        Sender {
            channel: channel.clone(),
        },
        Receiver { channel },
    )
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::default(),
        }
    }
}

impl<T> Sender<T> {
    /// Send a message through the channel.
    /// This method consumes the sender, ensuring that send is called once
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release)
    }
}

impl<T> Receiver<T> {
    /// Checks if a message is waiting in the channel
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    /// Receive the message from the channel, returning a T
    ///
    /// # Panics
    ///
    /// Panics when there is no messaged in the channel
    pub fn receive(self) -> T {
        if self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }

        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    /// Drops the channel.
    /// Additionally drops the message if it is waiting in the channel
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}
