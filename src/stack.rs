use std::ptr::null_mut;

#[derive(Debug)]
struct Node<T: Copy> {
    value: T,
    next: *mut Node<T>,
}

impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: null_mut(),
        }
    }

    fn get_value(&self) -> T {
        self.value.clone()
    }

    fn set_value(&mut self, v: T) {
        self.value = v;
    }

    fn get_next_mut(&self) -> Option<&mut Node<T>> {
        if self.next.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.next)
            }
        }
    }

    fn set_next(&mut self, node_ptr: *mut Node<T>) {
        self.next = node_ptr;
    }
}

pub struct Stack<T: Copy> {
    len: usize,
    head: *mut Node<T>,
    // tail: *mut Node<T>,
} 

impl<T: Copy> Stack<T> {
    pub fn new() -> Self {
        Stack {
            len: 0,
            head: null_mut(),
            // tail: null_mut(),
        }
    }

    pub fn push(&mut self, v: T) {
        let new_ptr: *mut Node<T> = Box::into_raw(Box::new(Node::new(v)));
        unsafe {
            (*new_ptr).set_next(self.head);
        }
        self.head = new_ptr;
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len < 1 { 
            None 
        } else {
            unsafe { 
                let val = (*self.head).get_value();
                let elem = Box::from_raw(self.head);
                let next = (*self.head).get_next_mut();
                match next {
                    None => { self.head = null_mut(); },
                    Some(next_ref) => { self.head = next_ref; },
                }
                self.len -= 1;
                Some(val)
            }
        }
    }

    pub fn top(&self) -> Option<T> {
        if self.len < 1 {
            None
        } else {
            unsafe {
                Some((*self.head).get_value())
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Copy> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut elem_ptr = self.head;
        for _ in 0..self.len {
            unsafe {
                let _elem = Box::from_raw(elem_ptr);
                elem_ptr = match (*elem_ptr).get_next_mut() {
                    Some(r) => { r as *mut Node<T> },
                    None => { null_mut() },
                }
            }
        }
    }
}
