pub mod tree;

pub use tree::Tree;
pub use tree::Node;

#[cfg(test)]
#[test]
fn tree_make() {
    let tree = Tree::new() as Tree<i32>;
    assert!(tree.is_empty());
}

#[test]
fn tree_insert() {
    let mut tree = tree::Tree::new();
    assert!(tree.is_empty());
    tree.insert(5);
    assert!(!tree.is_empty());
    match tree.root {
        Some(root) => assert_eq!(*root.get(), 5),
        None => panic!("Failed to insert")
    }
}

#[test]
fn print_test() {
    let mut tree = tree::Tree::new();
    assert!(tree.is_empty());
    tree.insert(5);
    assert!(!tree.is_empty());
    match &tree.root {
        &Some(ref root) => assert_eq!(*root.get(), 5),
        &None => panic!("Failed to insert")
    }
    tree.print();
}

#[test]
fn insert_test_balance() {
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

    if let Some(ref node) = tree.root {
        assert_eq!(5, *node.get());
        assert_eq!(Some(2), node.left().as_ref().map(|x| *x.get()));
        assert_eq!(4,node.height());
    }
    else {
        panic!("Tree not constructed")
    }
}
