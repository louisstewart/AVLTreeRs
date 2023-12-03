extern crate tree;

use tree::tree::Tree;

fn main() {
    let tree = (1..10).into_iter().collect::<Tree<_>>();
    assert!(!tree.is_empty());

    if let Some(ref node) = tree.root {
        println!("Root is {:?}", node.value());
        println!("Root left is {:?}", node.left().as_ref().map(|x| x.value()).unwrap());
    }
}
