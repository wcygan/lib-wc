use std::collections::LinkedList;

/// It's just a wrapper around std::ds::LinkedList which does
/// all of the heavy lifting
#[derive(Debug)]
pub struct BasicQueue<T> {
    elements: LinkedList<T>,
}

impl<T> BasicQueue<T> {
    pub fn new() -> BasicQueue<T> {
        BasicQueue {
            elements: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for BasicQueue<T> {
    fn default() -> BasicQueue<T> {
        BasicQueue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::BasicQueue;

    #[test]
    fn test_enqueue() {
        let mut queue: BasicQueue<u8> = BasicQueue::new();
        queue.enqueue(64);
        assert_eq!(queue.is_empty(), false);
    }

    #[test]
    fn test_dequeue() {
        let mut queue: BasicQueue<u8> = BasicQueue::new();
        queue.enqueue(32);
        queue.enqueue(64);
        let retrieved_dequeue = queue.dequeue();
        assert_eq!(retrieved_dequeue, Some(32));
    }

    #[test]
    fn test_peek_front() {
        let mut queue: BasicQueue<u8> = BasicQueue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        let retrieved_peek = queue.peek_front();
        assert_eq!(retrieved_peek, Some(&8));
    }

    #[test]
    fn test_size() {
        let mut queue: BasicQueue<u8> = BasicQueue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        assert_eq!(2, queue.len());
    }
}
