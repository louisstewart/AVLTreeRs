use std::fmt::Display;

pub struct Node<T> {
    value : T,
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>
}

pub struct Tree<T> {
    pub root : Option<Box<Node<T>>>
}

impl<T : PartialEq + PartialOrd + Display> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        !self.root.is_some()
    }

    pub fn print(&self) {
        match &self.root {
            &Some(ref node) => node.print(),
            &None => println!("Empty tree.")
        }
    }

    pub fn insert(&mut self, val : T) -> bool {
        let mut root = &mut self.root;
        match root {
            &mut Some(ref mut node) => node.insert(val),
            &mut None => {
                *root = Some(Box::new(Node::new(val)));
                true
            }
        }
    }
}

impl<T : PartialEq + PartialOrd + Display> Node<T> {
    pub fn new(val : T) -> Node<T> {
        Node { value : val, left : None, right : None }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn print(&self) {
        if let &Some(ref left) = &self.left {
            left.print();
        }
        println!("{}", self.value);
        if let &Some(ref right) = &self.right {
            right.print();
        }
    }

    pub fn insert(&mut self, val : T) -> bool {
        if val == self.value {
            return true
        }
        let t = if val < self.value { &mut self.left } else { &mut self.right };
        match t {
            &mut Some(ref mut target) => target.insert(val),
            &mut None => {
                *t = Some(Box::new(Node::new(val)));
                true
            }
        }
    }
}
