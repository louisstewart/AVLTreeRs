use std::fmt::Display;
use std::cmp;
use std::cmp::Ordering;

pub struct Node<T> {
    pub left : Option<Box<Node<T>>>,
    pub right : Option<Box<Node<T>>>,
    height : i32, // Balance factor for AVL tree.
    value : T,
}

pub struct Tree<T> {
    pub root : Option<Box<Node<T>>>
}

impl<T : PartialEq + Ord + Display> Tree<T> {
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
        match self.root.take() {
            Some(node) => {
                self.root = Some(insert(node,val));
                true
            },
            None => {
                self.root = Some(Box::new(Node::new(val)));
                true
            }
        }
    }

    pub fn contains(& self, val : T) -> bool {
        match &self.root {
            &Some(ref node) => {
                node.contains(val)
            },
            &None => {
                false
            }
        }
    }
}

impl<T : PartialEq + Ord + Display> Node<T> {
    pub fn new(val : T) -> Node<T> {
        Node { value : val, height : 1, left : None, right : None }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn left(& self) -> &Option<Box<Node<T>>> {
        &self.left
    }

    pub fn right(& self) -> &Option<Box<Node<T>>> {
        &self.right
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    fn bf(&self) -> i32 {
        self.left.as_ref().map(|x| x.height()).unwrap_or(0)
            - self.right.as_ref().map(|x| x.height()).unwrap_or(0)
    }

    fn update_height(&mut self) {
        self.height = cmp::max(self.left.as_ref().map(|x| x.height()).unwrap_or(0),
            self.right.as_ref().map(|x| x.height()).unwrap_or(0)) + 1
    }

    fn contains(& self, val : T) -> bool {
        match val.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                match &self.left {
                    &Some(ref node) => node.contains(val),
                    &None => false
                }
            }
            Ordering::Greater => {
                match &self.right {
                    &Some(ref node) => node.contains(val),
                    &None => false
                }
            }
        }
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
}

fn insert<T: PartialEq + Ord + Display>(mut node : Box<Node<T>>, val : T) -> Box<Node<T>> {
    match val.cmp((*node).get()) {
        Ordering::Equal => {
            *(*node).get_mut() = val;
            return node
        },
        Ordering::Less => node.left = insert_in(node.left.take(), val),
        Ordering::Greater => node.right = insert_in(node.right.take(), val)
    }
    node.update_height();
    restructure(node)
}

fn restructure<T: PartialEq + Ord + Display>(mut node : Box<Node<T>>) -> Box<Node<T>> {
    let balance = node.bf();
    match balance {
        -1...1 => return node,
        2 => {
            let left = node.left.take().expect("Wtf");
            if  subtree_height(&left.left) < subtree_height(&left.right) {
                node.left = Some(rotate_left(left));
                node.update_height();
            }
            else {
                node.left = Some(left);
            }
            rotate_right(node)
        },
        -2 => {
            let right = node.right.take().expect("Wtf");
            if subtree_height(&right.left) > subtree_height(&right.right) {
                node.right = Some(rotate_right(right));
                node.update_height();
            }
            else {
                node.right = Some(right);
            }
            rotate_left(node)
        },
        _ => panic!("Wtf")
    }
}

fn rotate_right<T: PartialEq + Ord + Display>(mut node : Box<Node<T>>) -> Box<Node<T>> {
    let mut b = node.left.take().expect("rotate right fail");
    node.left = b.right.take();
    (*node).update_height();
    b.right = Some(node);
    (*b).update_height();
    b
}

fn subtree_height<T: PartialEq + Ord + Display>(node : &Option<Box<Node<T>>>) -> i32 {
    node.as_ref().map_or(0, |x| x.height())
}

fn rotate_left<T: PartialEq + Ord + Display>(mut node : Box<Node<T>>) -> Box<Node<T>> {
    let mut b = node.right.take().expect("rotate left fail");
    node.right = b.left.take();
    (*node).update_height();
    b.left = Some(node);
    (*b).update_height();
    b
}

fn insert_in<T : PartialEq + Ord + Display>(node : Option<Box<Node<T>>>, val : T)
        -> Option<Box<Node<T>>> {
    match node {
        Some(n) => {
            Some(insert(n, val))
        },
        None => Some(Box::new(Node::new(val)))
    }
}
