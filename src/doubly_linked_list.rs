use std::ptr::null_mut;

#[derive(Debug)]
struct Node<T: Copy> {
    value: T, 
    next: *mut Self,
    prev: *mut Self,
}

impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: null_mut(),
            prev: null_mut(),
        }
    }

    fn get_value(&self) -> T {
        T::clone(&self.value)
    }

    fn set_value(&mut self, v: T) {
        self.value = v;
    }

    fn get_next(&self) -> *mut Self {
        self.next
    }

    fn set_next(&mut self, node_ptr: &mut Self) {
        self.next = node_ptr;
    }

    fn get_prev(&self) -> *mut Self {
        self.prev
    }

    fn set_prev(&mut self, node_ptr: &mut Self) {
        self.prev = node_ptr;
    }
} 

// impl<T: Copy> Drop for Node<T> {
//     fn drop(&mut self) {
//         println!("node dropped");
//     }
// }

#[derive(Debug)]
pub struct DoublyLinkedList<T: Copy> {
    len: usize,
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T: Copy> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            len: 0,
            head: null_mut(),
            tail: null_mut(),
        }
    }

    pub fn push_back(&mut self, v: T) {
        let new_ptr = Box::into_raw(Box::new(Node::new(v)));
        if self.tail.is_null() {
            assert_eq!(self.len, 0);
            self.head = new_ptr;
        } else {
            unsafe { 
                (*new_ptr).set_prev(&mut *self.tail);
                (*self.tail).set_next(&mut *new_ptr); 
            }
        }
        self.tail = new_ptr;
        self.len += 1;
    }

    pub fn push_front(&mut self, v: T) {
        let new_ptr = Box::into_raw(Box::new(Node::new(v)));
        if self.head.is_null() {
            assert_eq!(self.len, 0);
            self.tail = new_ptr;
        } else {
            unsafe { 
                (*new_ptr).set_next(&mut *self.head);
                (*self.head).set_prev(&mut *new_ptr); 
            }
        }
        self.head = new_ptr;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> T {
        if index >= self.len { panic!("out of bounds"); }
        unsafe {
            let mut elem_ptr = null_mut();
            if index <= self.len / 2 {
                elem_ptr = self.head;
                for _ in 0..index { elem_ptr = (*elem_ptr).get_next(); }
            } else {
                elem_ptr = self.tail;
                for _ in 0..(self.len-index-1) { elem_ptr = (*elem_ptr).get_prev(); }
            }
            (*elem_ptr).get_value()
        }
    }

    pub fn set(&mut self, index: usize, v: T) {
        if index == self.len { self.push_back(v) }
        else if index > self.len { panic!("out of bounds"); }
        unsafe {
            let mut elem_ptr = null_mut();
            if index <= self.len / 2 {
                elem_ptr = self.head;
                for _ in 0..index { elem_ptr = (*elem_ptr).get_next(); }
            } else {
                elem_ptr = self.tail;
                for _ in 0..(self.len-index-1) { elem_ptr = (*elem_ptr).get_prev(); }
            }
            (*elem_ptr).set_value(v);
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.len { panic!("out of bounds") }
        unsafe {
            let mut elem_ptr = null_mut();
            if index <= self.len / 2 {
                elem_ptr = self.head;
                for _ in 0..index { elem_ptr = (*elem_ptr).get_next(); }
            } else {
                elem_ptr = self.tail;
                for _ in 0..(self.len-index-1) { elem_ptr = (*elem_ptr).get_prev(); }
            }
            let prev_ptr = (*elem_ptr).get_prev();
            let next_ptr = (*elem_ptr).get_next();
            if !prev_ptr.is_null() { (*prev_ptr).set_next(&mut *next_ptr); }
            if !next_ptr.is_null() { (*next_ptr).set_prev(&mut *prev_ptr); }
            Box::from_raw(elem_ptr);
            // drop(from_raw(elem_ptr));
        }
        self.len -= 1;
    }

    pub fn insert(&mut self, index: usize, v: T) {
        if index == 0 { self.push_front(v) }
        else if index == self.len { self.push_back(v) }
        else if index > self.len { panic!("out of bounds"); }
        let new_ptr = Box::into_raw(Box::new(Node::new(v)));
        unsafe {
            let mut elem_ptr = null_mut();
            if index <= self.len / 2 {
                elem_ptr = self.head;
                for _ in 0..index { elem_ptr = (*elem_ptr).get_next(); }
            } else {
                elem_ptr = self.tail;
                for _ in 0..(self.len-index-1) { elem_ptr = (*elem_ptr).get_prev(); }
            }
            let prev_ptr = (*elem_ptr).get_prev();
            (*prev_ptr).set_next(&mut *new_ptr);
            (*new_ptr).set_next(&mut *elem_ptr);
            (*new_ptr).set_prev(&mut *prev_ptr);
            (*elem_ptr).set_prev(&mut *new_ptr);
        }
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Copy> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        if self.len == 0 { return; }
        else if self.len == 1 {
            unsafe { Box::from_raw(self.head); }
        }
        else {
            unsafe {
                let mut elem_ptr = self.head;
                let mut next_ptr = (*self.head).get_next();
                for _ in 0..self.len {
                    Box::from_raw(elem_ptr);
                    elem_ptr = next_ptr;
                    if !next_ptr.is_null() { next_ptr = (*next_ptr).get_next(); }
                }
            }
        }
    }
}
