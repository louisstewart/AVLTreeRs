pub mod tree;

pub use tree::Tree;
pub use tree::Node;

#[cfg(test)]
#[test]
fn tree_make() {
    let tree = tree::Tree::new();
    assert!(tree.is_empty());
}

#[test]
fn tree_insert() {
    let mut tree = tree::Tree::new();
    assert!(tree.is_empty());
    tree.insert(5);
    assert!(!tree.is_empty());
    match tree.root {
        Some(root) => assert_eq!(root.get(), 5),
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
        &Some(ref root) => assert_eq!(root.get(), 5),
        &None => panic!("Failed to insert")
    }
    tree.print();
}
