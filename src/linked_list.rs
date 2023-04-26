#[derive(Debug)]
struct Node<T: Copy> {
    data: Box<T>,
    next: *mut Node<T>,
}

impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        let data: Box<T> = Box::new(value);
        Node {
            data,
            next: std::ptr::null_mut(),
        }
    }

    fn get_value(&self) -> T {
        *(self.data).clone()
    }

    fn set_value(&mut self, data: T) {
        self.data = Box::new(data);
    }

    fn get_next(&self) -> Option<&Node<T>> {
        unsafe {
            self.next.as_ref()
        }
    }

    fn get_next_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            self.next.as_mut()
        }
    }

    fn set_next(&mut self, node_ptr: *mut Node<T>) {
        self.next = node_ptr;
    }
}

#[derive(Debug)]
pub struct LinkedList<T: Copy> {
    len: usize,
    first: *mut Node<T>,
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            len: 0,
            first: std::ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, data: T) {
        let node_ptr: *mut Node<T> = Box::into_raw(Box::new(Node::new(data)));
        assert!(!node_ptr.is_null());
        if self.first.is_null() { // self.len == 0
            self.first = node_ptr;
        } else {
            let elem_ptr = self.first;
            self.first = node_ptr;
            unsafe {
                (*self.first).next = elem_ptr;
            }
        }
        self.len += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let node_ptr: *mut Node<T> = Box::into_raw(Box::new(Node::new(data)));
        let last = self.get_last_mut();
        match last {
            None => { self.first = node_ptr; }, // self.len == 0
            Some(_) => { last.unwrap().set_next(node_ptr); },
        };
        self.len += 1;
    }

    fn get_last_mut(&mut self) -> Option<&mut Node<T>> {
        if self.first.is_null() { return None; }
        let mut elem_ptr = self.first;
        unsafe {
            let mut next = (*elem_ptr).get_next_mut();
            while next.is_some() {
                elem_ptr = next.unwrap();
                next = (*elem_ptr).get_next_mut();
            }
            elem_ptr.as_mut()
        }
    }

    pub fn set(&mut self, index: usize, data: T) {
        if index >= self.len { panic!("out of bounds"); }
        let mut elem_ptr = self.first;
        unsafe {
            for _ in 0..index {
                elem_ptr = (*elem_ptr).get_next_mut().unwrap();
            }
            assert!(!elem_ptr.is_null());
            (*elem_ptr).set_value(data);
        }
    }

    pub fn get(&self, index: usize) -> T {
        if index >= self.len { panic!("out of bounds"); }
        let mut elem_ptr = self.first;
        unsafe {
            for _ in 0..index {
                elem_ptr = (*elem_ptr).get_next_mut().unwrap();
            }
            assert!(!elem_ptr.is_null());
            (*elem_ptr).get_value()
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.len { panic!("out of bounds"); }
        let mut elem_ptr = self.first;
        let mut prev_ptr: *mut Node<T> = std::ptr::null_mut();
        unsafe {
            for _ in 0..index {
                prev_ptr = elem_ptr;
                elem_ptr = (*elem_ptr).get_next_mut().unwrap();
            }
            let next = match (*elem_ptr).get_next_mut() {
                None => std::ptr::null_mut(),
                Some(node) => node as *mut Node<T>,
            };
            if prev_ptr.is_null() { // index == 0
                self.first = next;
            } else {
                (*prev_ptr).set_next(next);
            }
            // std::ptr::drop_in_place(elem_ptr);
            Box::from_raw(elem_ptr);
        }
        self.len -= 1;
    }

    pub fn insert(&mut self, index: usize, data: T) {
        if index > self.len { panic!("out of bounds"); }
        else if index == self.len { self.push_back(data) }
        else if index == 0 { self.push_front(data) }

        let new_ptr = Box::into_raw(Box::new(Node::new(data)));
        let mut elem_ptr = self.first;
        let mut prev_ptr: *mut Node<T> = std::ptr::null_mut();
        unsafe {
            for _ in 0..index {
                prev_ptr = elem_ptr;
                elem_ptr = (*elem_ptr).get_next_mut().unwrap();
            }
            (*prev_ptr).set_next(new_ptr);
            (*new_ptr).set_next(elem_ptr);
        }
        self.len += 1;
    }
}

impl<T: Copy> Drop for LinkedList<T> {
    fn drop(&mut self) {
        if self.len ==  0 { return; }
        else if self.len == 1 {
            unsafe {
                Box::from_raw(self.head);
            }
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

// impl<T> FromIterator<T> for LinkedLisw<T> {
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> LinkedList<T> {

//     }
// }
