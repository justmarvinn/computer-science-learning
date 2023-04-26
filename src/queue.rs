use std::ptr::{ null_mut, addr_of_mut, drop_in_place };
use std::alloc::{ alloc, dealloc, Layout, handle_alloc_error };

struct Node<T: Clone> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T: Clone> Node<T> {
    fn new_ptr(value: T) -> *mut Self {
        let layout = Layout::new::<Node<T>>();
        unsafe {
            let node_ptr = alloc(layout) as *mut Node<T>;
            if node_ptr.is_null() {
                handle_alloc_error(layout);
            }
            (*node_ptr).value = value;
            (*node_ptr).next = null_mut();
            (*node_ptr).prev = null_mut();
            node_ptr
        }
    }

    fn get_next(&self) -> *mut Node<T> {
        self.next
    }

    fn set_next(&mut self, node: &mut Node<T>) {
        self.next = node as *mut Node<T>;
    }

    fn get_prev(&self) -> *mut Node<T> {
        self.prev
    }

    fn set_prev(&mut self, node: &mut Node<T>) {
        self.prev = node as *mut Node<T>;
    }

    fn get_value(&self) -> T {
        T::clone(&self.value)
    }
}

impl<T: Clone> Drop for Node<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<Node<T>>();
        unsafe {
            dealloc((self as *mut Node<T>) as *mut u8, layout);
        }
    }
}

pub struct Queue<T: Clone> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_ptr = Node::new_ptr(value);
        if self.len == 0 {
            self.head = new_ptr;
            self.tail = new_ptr;
        } else {
            unsafe {
                (*new_ptr).set_next(&mut *self.head);
                (*self.head).set_prev(&mut *new_ptr);
                self.head = new_ptr;
            }
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len < 1 { 
            None
        } else {
            unsafe {
                let res = (*self.tail).get_value();
                let tail_ptr = (*self.tail).get_prev();
                drop_in_place(self.tail);
                self.tail = tail_ptr;
                self.len -= 1;
                Some(res)
            }
        }
    }

    pub fn peek(&self) -> Option<T> {
        if self.len < 1 {
            None
        } else {
            unsafe {
                Some((*self.head).get_value())
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // симметричный обход
    pub fn infix_traverse(&self) -> Vec<T> {}

    pub fn prefix_traverse(&self) -> Vec<T> {}
    pub fn postfix_traverse(&self) -> Vec<T> {}
}

impl<T: Clone> Drop for Queue<T> {
    fn drop(&mut self) {
        let mut node_ptr = self.head;
        for _ in 0..self.len {
            unsafe {
                let next_ptr = (*node_ptr).get_next();
                drop_in_place(node_ptr);
                node_ptr = next_ptr;
            }
        }
    }
}
