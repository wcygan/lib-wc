//! Multiple producer, multiple consumer channels
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

/// An unbounded channel that allows multiple producers and multiple consumers
pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

/// An unbounded channel that allows multiple producers and multiple consumers
impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    /// Add a message to the channel
    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    /// Retrieve a message from the channel. If there are no messages, this will block until one is available.
    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }

            b = self.item_ready.wait(b).unwrap();
        }
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Channel::new()
    }
}

#[cfg(test)]
mod tests {
    use std::thread::scope;

    use super::*;

    #[test]
    fn test_channel_is_emptied() {
        // since we have an equal number of sends and receives, the channel should be empty at the end
        // hence, this test should not deadlock (e.g., since receive is a blocking operation)
        let channel: Channel<u32> = Channel::new();
        let (lo, hi) = (0, 100);

        scope(|s| {
            s.spawn(|| {
                for i in lo..hi {
                    channel.send(i);
                }
            });

            s.spawn(|| {
                for _ in lo..hi {
                    channel.receive();
                }
            });
        });
    }
}
