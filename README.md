# AVLTreeRs

This is an implementation of the AVL algorithm for self-balancing
binary trees in Rust. It is an exercise in learning to use Rust, using dynamically allocated mem for on heap storage of nodes to create a recursive data structure.

## Struct API

2 structs are created in this module, the generic data type structs of **Tree** and **Node**.
The AVL tree struct stores the root **Node** and also presents the forward facing API for
interacting with the data structure. The methods of the Node struct are private to Node.

1. Creation

  **Tree** is a generic data structure for all types that implement `PartialEq + Ord + Display` traits. Instantiate a new tree using the `Tree::new()` constructor and cast to appropriate type.

  ```rust
  let mut tree = Tree::new() as Tree<i32>
  ```
For instance, this is a new tree of 32bit ints.

2. Insertion

  After creation, insert items of whatever type the tree takes into the tree with `insert(T)`. Method returns `true` if the insertion was successful.

  ```rust
  tree.insert(4);
  tree.insert(1);
  tree.insert(2);
  tree.insert(5);
  // New root should be 2
  if let Some(ref node) = tree.root {
      assert!(2, *node.get())
  }
  ```

3. Searching

  Use the `contains(T)` method to search for a value in the tree. Will return true if the value is there - otherwise false. Since the tree is self-balancing, searching is an O(log n) operation.

  ```rust
  let x: bool = tree.contains(5);
  assert!(x) // Succeeds if 5 was inserted
  ```

4. Deletion

  The `delete(T)` method is used to remove values from the tree if they exist. If the tree is empty then the call returns false, otherwise it returns true, even if the value does not exist in the tree (may change in future).

  ```rust
  // Assuming values above inserted
  let x: bool = tree.delete(4) // true

  ```
