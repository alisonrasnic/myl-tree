# myl-tree
## A basic binary tree implementation in Rust using unsafe NonNull's with safe public API

## Usage
```rust

import myl_tree::{Tree, TreeNode};

let tree = Tree::new();

let head = TreeNode::new(1);

tree.set_head(head);

let node_l = TreeNode::new(-1);

head.set_left(node_l);

assert_eq!(tree.search_vlr(-1).unwrap().cmp_ptr(&node_l), true);

```
