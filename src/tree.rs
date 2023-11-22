use std::cmp;
use std::cmp::Ordering;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Node<T: Ord + PartialEq> {
    pub left : Option<Box<Node<T>>>,
    pub right : Option<Box<Node<T>>>,
    height : i32, // Balance factor for AVL tree.
    value : T,
}

#[derive(Debug)]
pub struct Tree<T: Ord + PartialEq> {
    pub root : Option<Box<Node<T>>>
}

#[derive(Debug)]
pub struct TreeIter<'a, T: Ord + PartialEq> {
    previous_nodes: Vec<&'a Node<T>>,
    current_node: &'a Option<Box<Node<T>>>
}

impl<'a, T: 'a + Ord + PartialEq> Iterator for TreeIter<'a, T> {

    type Item = &'a Node<T>;

    fn next(& mut self) -> Option<Self::Item> {
        loop {
            match *self.current_node {
                Some(ref node) => {
                    if node.left.is_some() {
                        self.previous_nodes.push(node);
                        self.current_node = &node.left;
                        continue
                    }
                    if node.right.is_some() {
                        self.current_node = &node.right;
                        return Some(node)
                    }

                    self.current_node = &None;
                    return Some(&**node);
                },
                None => match self.previous_nodes.pop() {
                    None => return None,
                    Some( n) => {
                        self.current_node = &n.right;
                        return Some(n);
                    }
                }
            }
        }
    }
}

impl<'a, T: 'a + Ord + PartialEq> Tree<T> {
    pub fn node_iter(&'a self) -> TreeIter<'a, T> {
        TreeIter {
            previous_nodes: Vec::new(),
            current_node: &self.root,
        }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.node_iter().map(|node| &node.value)
    }
}

impl<T : PartialEq + Ord> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
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

    pub fn delete(&mut self, val: T) -> bool {
        match self.root.take() {
            Some(node) => {
                self.root = delete(Some(node), val);
                true
            },
            None => {
                self.root = None;
                false
            }
        }
    }
}

impl<T: Ord + PartialEq> FromIterator<T> for Tree<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut tree = Self::new();

        for i in iter {
            tree.insert(i);
        }

        tree
    }
}

impl<T : PartialEq + Ord> Node<T> {
    pub fn new(val : T) -> Node<T> {
        Node { value : val, height : 1, left : None, right : None }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn left(&self) -> &Option<Box<Node<T>>> {
        &self.left
    }

    pub fn right(&self) -> &Option<Box<Node<T>>> {
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
}

fn insert<T: PartialEq + Ord>(mut node : Box<Node<T>>, val : T) -> Box<Node<T>> {
    match val.cmp((*node).value()) {
        Ordering::Equal => {
            *(*node).value_mut() = val;
            return node
        },
        Ordering::Less => node.left = insert_in(node.left.take(), val),
        Ordering::Greater => node.right = insert_in(node.right.take(), val)
    }
    node.update_height();
    restructure(node)
}

fn delete<T: PartialEq + Ord>(node: Option<Box<Node<T>>>, val : T) -> Option<Box<Node<T>>> {
    match node {
        None => None,
        Some(mut n) => {
            match val.cmp((*n).value()) {
                Ordering::Less => n.left = delete(n.left.take(), val),
                Ordering::Greater => n.right = delete(n.right.take(), val),
                Ordering::Equal => match remove_in(n) {
                        Some(m) => n = m,
                        None => return None
                    }
            }
            n.update_height();
            Some(restructure(n))
        }
    }
}

fn restructure<T: PartialEq + Ord>(mut node : Box<Node<T>>) -> Box<Node<T>> {
    let balance = node.bf();
    match balance {
        -1..=1 => return node,
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

fn rotate_right<T: PartialEq + Ord>(mut node : Box<Node<T>>) -> Box<Node<T>> {
    let mut b = node.left.take().expect("rotate right fail");
    node.left = b.right.take();
    (*node).update_height();
    b.right = Some(node);
    (*b).update_height();
    b
}

fn subtree_height<T: PartialEq + Ord>(node : &Option<Box<Node<T>>>) -> i32 {
    node.as_ref().map_or(0, |x| x.height())
}

fn rotate_left<T: PartialEq + Ord>(mut node : Box<Node<T>>) -> Box<Node<T>> {
    let mut b = node.right.take().expect("rotate left fail");
    node.right = b.left.take();
    (*node).update_height();
    b.left = Some(node);
    (*b).update_height();
    b
}

fn minimum<T: PartialEq + Ord>(node: Box<Node<T>>) -> Box<Node<T>>{
    match node.left {
        Some(n) => minimum(n),
        None => node
    }
}

fn insert_in<T : PartialEq + Ord>(node : Option<Box<Node<T>>>, val : T)
        -> Option<Box<Node<T>>> {
    match node {
        Some(n) => {
            Some(insert(n, val))
        },
        None => Some(Box::new(Node::new(val)))
    }
}

/*
 * 3 cases:
 *  1) no kids, so remove returns none
 *  2) only left or right kid, so replace
 *  3) 2 kids, so find min greater
 */
fn remove_in<T: PartialEq + Ord>(mut node : Box<Node<T>>)
        -> Option<Box<Node<T>>> {
    if node.left.is_none() && node.right.is_none() {
        return None;
    }
    else if node.left.is_none() && !node.right.is_none() {
        return node.right;
    }
    else if node.right.is_none() && !node.left.is_none() {
        return node.left;
    }
    else {
        // Get in-order successor
        let mut successor = minimum(node.right.take().unwrap());
        successor.left = node.left;
        successor.right = node.right;
        return Some(successor)
    }
}
