# AVLTreeRs

This will eventually be an implementation of the AVL algorithm for self-balancing
binary trees in Rust. It is an exercise in learning to use Rust, and also a decent
data structure to implement.

## Layout

2 structs are created in this module, the generic data type structs of **Tree** and **Node**.
As well as implemented methods on these structs to support insertion and printing.

```rust
let mut tree = Tree::new() as Tree<i32>
tree.insert(5);
tree.print();
```
