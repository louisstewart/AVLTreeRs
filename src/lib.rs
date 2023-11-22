pub mod tree;

pub use tree::Tree;
pub use tree::Node;

use std::collections::BTreeSet;
use itertools::equal;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
extern crate itertools;

#[test]
fn tree_make() {
    let tree: Tree<i32> = Tree::new();
    assert!(tree.is_empty());
}

#[test]
fn tree_insert() {
    let mut tree = Tree::new();
    assert!(tree.is_empty());
    tree.insert(5);
    assert!(!tree.is_empty());
    match tree.root {
        Some(root) => assert_eq!(*root.value(), 5),
        None => panic!("Failed to insert")
    }
}

#[test]
fn insert_test_balance() {
    let mut tree = Tree::new();
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
        assert_eq!(5, *node.value());
        assert_eq!(2, *node.left().as_ref().unwrap().value());
        assert_eq!(4, node.height());
    }
    else {
        panic!("Tree not constructed")
    }
}

#[test]
fn search_test() {
    let mut tree = Tree::new();
    assert!(tree.is_empty());
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(5);

    if let Some(ref node) = tree.root {
            assert_eq!(3, node.height());
    }

    assert!(tree.contains(2))
}

#[test]
fn test_remove() {
    let mut tree = Tree::new();
    tree.insert(4);
    tree.insert(3);
    tree.insert(2);
    tree.insert(1);

    assert!(tree.delete(2));
    assert!(!tree.contains(2));

    assert!(tree.delete(3));
    assert!(!tree.contains(3));

    assert!(tree.delete(4));
    assert!(!tree.contains(4));

    assert!(tree.delete(1));
    assert!(!tree.contains(1));
}

#[quickcheck]
fn test_with_big_set(xs: Vec<i32>) -> bool {
    let mut canonical_set = xs.iter().cloned().collect::<BTreeSet<_>>();
    let mut tree_set = Tree::new();
    xs.iter().cloned().for_each(|x| -> () {tree_set.insert(x);});
    xs.iter().cloned().all(|x| -> bool {tree_set.contains(x)});
    equal(canonical_set.iter(), tree_set.iter());

    let mut all_equal = true;
    for x in xs {
        tree_set.delete(x);
        canonical_set.remove(&x);
        all_equal &= equal(canonical_set.iter(), tree_set.iter())
    }

    all_equal
}
