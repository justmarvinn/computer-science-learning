use std::ptr::{ null_mut, drop_in_place };
use std::alloc::{ alloc, dealloc, Layout, handle_alloc_error };
use std::cmp::Ordering;
use std::fmt;

struct Node<T>
where
    T: Copy + Eq + Ord
{
    value: T,
    parent: *mut Self,
    left: *mut Self,
    right: *mut Self,
}

impl<T> Node<T> 
where
    T: Copy + Ord + fmt::Display
{
    // fn new(value: T) -> Self {
    //     Node {
    //         value,
    //         left: null_mut(),
    //         right: null_mut(),
    //     }
    // }

    fn new_ptr(value: T) -> *mut Self {
        let layout = Layout::new::<Node<T>>();
        unsafe {
            let node_ptr = alloc(layout) as *mut Node<T>;
            if node_ptr.is_null() {
                handle_alloc_error(layout);
            }
            (*node_ptr).value = value;
            (*node_ptr).parent = null_mut();
            (*node_ptr).left = null_mut();
            (*node_ptr).right = null_mut();
            node_ptr
        }
    }

    fn get_value(&self) -> T {
        T::clone(&self.value)
    }    

    fn set_left(&mut self, node: &mut Self) -> Result<(), String> {
        if !self.left.is_null() { 
            Err("Node already has a left child".to_string())
        } else {
            node.parent = self;
            self.left = node as *mut Self;
            Ok(())
        }
    }

    fn set_right(&mut self, node: &mut Self) -> Result<(), String> {
        if !self.right.is_null() {
            Err("Node already has a right child".to_string())
        } else {
            node.parent = self;
            self.right = node as *mut Self;
            Ok(())
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_null() && self.right.is_null()
    }

    fn go_left_recursively(&self) -> &Self {
        if self.left.is_null() {
            &self
        } else {
            unsafe {
                (*self.left).go_left_recursively()
            }
        }
    }

    fn go_right_recursively(&self) -> &Self {
        if self.right.is_null() {
            &self
        } else {
            unsafe {
                (*self.right).go_right_recursively()
            }
        }
    }

    fn search(&self, value: T) -> Option<&Self> {
        unsafe {
            match self.value.cmp(&value) {
                Ordering::Less => if self.right.is_null() { None } else { (*self.right).search(value) },
                Ordering::Equal => Some(self),
                Ordering::Greater => if self.left.is_null() { None } else { (*self.left).search(value) },
            }
        }
    }

    fn add(&mut self, node: &mut Self) {
        match self.value.cmp(&node.value) {
            Ordering::Less | Ordering::Equal => {
                if self.right.is_null() { 
                    self.set_right(node).unwrap(); 
                } else {
                    unsafe {
                        (*self.right).add(node);
                    }
                }
            },
            Ordering::Greater => {
                if self.left.is_null() {
                    self.set_left(node).unwrap();
                } else {
                    unsafe {
                        (*self.left).add(node);
                    }
                }
            },
        };
    }

    fn del(&mut self, value: T) -> Result<(), String> {
        unsafe { 
            match self.value.cmp(&value) {
                Ordering::Less => { 
                    if self.right.is_null() { 
                        Err("nothing to delete".to_string()) 
                    } else { 
                        (*self.right).del(value)
                    }
                },
                Ordering::Greater =>  {
                    if self.left.is_null() {
                        Err("nothing to delete".to_string()) 
                    } else { 
                        (*self.left).del(value)
                    }
                },
                Ordering::Equal => {
                    if self.is_leaf() {
                        if self.value < (*self.parent).value {
                            (*self.parent).left = null_mut();
                        } else {
                            (*self.parent).right = null_mut();
                        }
                        drop(self);
                    } else if !self.left.is_null() && self.right.is_null() {
                        (*self.left).parent = self.parent;
                        if (*self.left).value < (*self.parent).value {
                            (*self.parent).left = self.left;
                        } else {
                            (*self.parent).right = self.left;
                        }
                        drop(self);
                    } else if self.left.is_null() && !self.right.is_null() {
                        (*self.right).parent = self.parent;
                        if (*self.right).value < (*self.parent).value {
                            (*self.parent).left = self.right;
                        } else {
                            (*self.parent).right = self.right;
                        }
                        drop(self);
                    } else {
                        // if (*self.left).is_leaf() {
                        //     self.value = (*self.left).value;
                        //     (*self.left).del(self.value).unwrap();
                        // } else {
                            let node_ptr = (*self.right).go_left_recursively();
                            self.value = (*node_ptr).value;
                            (*self.right).del(self.value).unwrap();
                        // }
                    }
                    Ok(())
                },
            }
        }
    }

    fn infix_traverse(&self, vec: &mut Vec<T>) {
        if !self.left.is_null() {
            unsafe {
                (*self.left).infix_traverse(vec);
            }
        }
        vec.push(self.get_value());
        if !self.right.is_null() {
            unsafe {
                (*self.right).infix_traverse(vec);
            }
        }
    }

    fn prefix_traverse(&self, vec: &mut Vec<T>) {
        vec.push(self.get_value());
        if !self.left.is_null() {
            unsafe {
                (*self.left).prefix_traverse(vec);
            }
        }
        if !self.right.is_null() {
            unsafe {
                (*self.right).prefix_traverse(vec);
            }
        }
    }

    fn postfix_traverse(&self, vec: &mut Vec<T>) {
        if !self.left.is_null() {
            unsafe {
                (*self.left).postfix_traverse(vec);
            }
        }
        if !self.right.is_null() {
            unsafe {
                (*self.right).postfix_traverse(vec);
            }
        }
        vec.push(self.get_value());
    }

    fn print_traverse(&self, string: &mut String, indent: usize) {
        string.push_str(&format!("{}{}\n", " ".repeat(indent*2), self.value));
        if !self.left.is_null() {
            unsafe {
                (*self.left).print_traverse(string, indent+1);
            }
        }
        if !self.right.is_null() {
            unsafe {
                (*self.right).print_traverse(string, indent+1);
            }
        }
    }
}

impl<T> Drop for Node<T>
where
    T: Copy + Ord
{
    fn drop(&mut self) {
        unsafe {
            if !self.left.is_null() { drop_in_place(self.left); }
            if !self.right.is_null() { drop_in_place(self.right); }
            let layout = Layout::new::<Node<T>>();
            dealloc((self as *mut Node<T>) as *mut u8, layout);
        }
    }
}

pub struct BinarySearchTree<T>
where
    T: Copy + Ord
{
    root: *mut Node<T>,
    num_elements: usize,
}

impl<T> BinarySearchTree<T>
where
    T: Copy + Ord + fmt::Display
{
    pub fn new() -> Self {
        BinarySearchTree {
            root: null_mut(),
            num_elements: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        let new_ptr = Node::new_ptr(value);
        if self.root.is_null() {
            self.root = new_ptr;
        } else {
            unsafe {
                (*self.root).add(&mut *new_ptr);
            }
        }
        self.num_elements += 1;
    }

    pub fn contents(&self, value: T) -> bool {
        if self.root.is_null() {
            assert_eq!(self.num_elements, 0);
            false
        } else {
            unsafe {
                (*self.root).search(value).is_some()
            } 
        }
    }

    pub fn remove(&mut self, value: T) -> Result<(), String> {
        unsafe {
            if self.root.is_null() {
                assert_eq!(self.num_elements, 0);
                Err("tree is already empty".to_string())
            } else if self.num_elements == 1 {
                drop_in_place(self.root);
                self.root = null_mut();
                self.num_elements -= 1;
                Ok(())
            } else if value == (*self.root).value &&
                !(!(*self.root).left.is_null() && !(*self.root).right.is_null())
            {
                let tmp = self.root;
                if (*self.root).left.is_null() {
                    self.root = (*self.root).right;
                } else if (*self.root).right.is_null() {
                    self.root = (*self.root).left;
                }
                (*tmp).left = null_mut();
                (*tmp).right = null_mut();
                drop_in_place(tmp);
                self.num_elements -= 1;
                Ok(())
            } else {
                self.num_elements -= 1;
                (*self.root).del(value)
            }
        }
    }

    pub fn min(&self) -> Option<T> {
        if self.root.is_null() {
            None
        } else {
            unsafe {
                Some( (*self.root).go_left_recursively().get_value() )
            }
        }
    }
    
    pub fn max(&self) -> Option<T> {
        if self.root.is_null() {
            assert_eq!(self.num_elements, 0);
            None
        } else {
            unsafe {
                Some( (*self.root).go_right_recursively().get_value() )
            }
        }
    }

    pub fn count(&self) -> usize {
        self.num_elements
    }

    pub fn infix_traverse(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.num_elements);
        if !self.root.is_null() {
            unsafe {
                (*self.root).infix_traverse(&mut res);
            }
        }
        res
    }

    pub fn prefix_traverse(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.num_elements);
        if !self.root.is_null() {
            unsafe {
                (*self.root).prefix_traverse(&mut res);
            }
        }
        res
    }

    pub fn postfix_traverse(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.num_elements);
        if !self.root.is_null() {
            unsafe {
                (*self.root).postfix_traverse(&mut res);
            }
        }
        res
    }
}

impl<T> Drop for BinarySearchTree<T> 
where
    T: Copy + Ord
{
    fn drop(&mut self) {
        if !self.root.is_null() {
            unsafe { drop_in_place(self.root); }
        }
    }
}

impl<T> fmt::Display for BinarySearchTree<T> 
where
    T: Copy + Ord + fmt::Display
{
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.num_elements == 0 {
            writeln!(f, "empty tree")
        } else {
            unsafe {
                let mut out = String::new();
                (*self.root).print_traverse(&mut out, 0);
                write!(f, "{}", &out[..out.len()-1])
            }
        }
    }
}
