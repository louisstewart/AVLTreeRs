extern crate tree;

use tree::Tree;

fn main() {
    let mut tree = Tree::new() as Tree<i32>;
    assert!(tree.is_empty());
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(5);
    tree.insert(6);
    tree.insert(9);
    tree.insert(14);
    tree.insert(11);
    tree.insert(10);
    tree.insert(20);
    assert!(!tree.is_empty());
    tree.print();

    if let Some(ref node) = tree.root {
        println!("Root is {:?}", node.get());
        println!("Root left is {:?}", node.left().as_ref().map(|x| x.get()));
    }
    tree.print();
}
