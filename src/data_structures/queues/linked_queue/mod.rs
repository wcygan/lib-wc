use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}

pub struct LinkedQueue<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T> LinkedQueue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    pub fn enqueue(&mut self, data: T) {
        let node = Box::new(Node::new(data));
        let node = NonNull::new(Box::into_raw(node)).unwrap();
        match self.tail {
            Some(tail) => unsafe {
                (*tail.as_ptr()).next = Some(node);
            },
            None => self.head = Some(node),
        }
        self.tail = Some(node);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.map(|head| unsafe {
            let head = Box::from_raw(head.as_ptr());
            self.head = head.next;
            if self.head.is_none() {
                self.tail = None;
            }
            head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|head| unsafe { &(*head.as_ptr()).data })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut node = self.head;
        while let Some(n) = node {
            len += 1;
            unsafe {
                node = (*n.as_ptr()).next;
            }
        }
        len
    }
}

impl<T> Default for LinkedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_peeking() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.peek(), Some(&3));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut queue = LinkedQueue::new();
        assert!(queue.is_empty());
        queue.enqueue(1);
        assert!(!queue.is_empty());
        queue.dequeue();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_len() {
        let mut queue = LinkedQueue::new();
        assert_eq!(queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.enqueue(3);
        assert_eq!(queue.len(), 3);
        queue.dequeue();
        assert_eq!(queue.len(), 2);
        queue.dequeue();
        assert_eq!(queue.len(), 1);
        queue.dequeue();
        assert_eq!(queue.len(), 0);
    }
}
