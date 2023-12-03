extern crate tree;

use tree::tree::Tree;

fn main() {
    let mut tree = Tree::new() as Tree<i32>;
    assert!(tree.is_empty());
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    tree.insert(5);
    tree.insert(6);
    tree.insert(7);
    tree.insert(8);
    tree.insert(9);
    tree.insert(10);
    assert!(!tree.is_empty());

    if let Some(ref node) = tree.root {
        println!("Root is {:?}", node.value());
        println!("Root left is {:?}", node.left().as_ref().map(|x| x.value()).unwrap());
    }
}
