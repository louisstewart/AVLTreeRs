extern crate tree;

use tree::Tree;
use tree::Node;

use std::io;
use std::cmp::Ordering;

fn main() {
    let mut tree = Tree::new() as Tree<i32>;
    assert!(tree.is_empty());
    tree.insert(5);
    tree.insert(6);
    tree.insert(1);
    tree.insert(2);
    tree.insert(10);
    assert!(!tree.is_empty());
    tree.print();

    if let Some(ref node) = tree.root {
        println!("{:?}", node.get());
    }
    tree.print();
}
