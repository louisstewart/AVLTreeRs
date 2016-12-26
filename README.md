# AVLTreeRs

This will eventually be an implementation of the AVL algorithm for self-balancing
binary trees in Rust. It is an exercise in learning to use Rust, and also a decent
data structure to implement.

## Api

2 structs are created in this module, the generic data type structs of **Tree** and **Node**.
The AVL tree struct stores the root **Node** and also presents the forward facing API for
interacting with the data structure. The methods of the Node struct are private to Node.

1. Creation

  **Tree** is a generic data structure for all types that implement `PartialEq + Ord + Display` traits. Instantiate a new tree the `new(T)` constructor and cast to appropriate type.
```rust
let mut tree = Tree::new() as Tree<i32>
```

2. Insertion

  After creation, insert items of whatever type the tree takes into the tree with `insert(T)`. Method returns `true` if the insertion was successful.

  ```rust
  tree.insert(4);
  tree.insert(1);
  tree.insert(2);
  tree.insert(5);
  ```

3. Searching

  Use the `contains` method to search for a value in the tree. Will return true if the value is there - otherwise false. Since the tree is self-balancing, searching is an O(log n) operation.

  ```rust
  let x: bool = tree.contains(5);
  assert!(x) // Succeeds if 5 was inserted
  ```
