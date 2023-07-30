// Module to try out pointer (and box) arithmetic in rust
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { 
            val: t, 
            next: None, 
            prev: None
        }
    }
}

pub struct LinkedList<T> {
    length: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { 
            length: 0, 
            head: None, 
            tail: None, 
            marker: PhantomData 
        }
    }

    pub fn insert_at_head(&mut self, obj: T) {
        // Encapsulate new Value into node and box-pointer
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.tail;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe {
                (*tail_ptr.as_ptr()).next = node_ptr
            }
        }
        self.tail = node_ptr;
        self.length += 1;
    }
}