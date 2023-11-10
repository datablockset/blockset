# Glossary

- **`content-hash`**, **`block-hash`** is a hash of a content or a data block. A `content-hash` is in `base32` format.
- **`content-dependent-tree`**, **`CDT`**. The CDT nodes can be split as a `storage-tree` or `main-tree`.
- **`CDT0`** is the name of a hash function that is based on a `content-dependent tree` and the `SHA-224` compression function.
- **`storage-tree`**, **`forest-tree`**, **`blockset-tree`**, **`block-tree`** is a tree in a `forest`.  In particular, a `blockset` storage keeps its nodes in files. One node is one file. Each storage may have its own size limitation for one node. However, the split algorithm should align levels and sublevels to `2^n`. In this case, a storage with a smaller limit will always have blocks for a storage with a bigger limit. The tree uses only hashes (`224 bits`) as node ids. A value of a forrest node contains a `block` of data.
- **`forest`**, **`tree-storage`** is a storage that keeps trees.
- **`forest-node-id`** is a hash of a node of data. Contains
    - **`type`** is either `root` or `child`.
    - **`hash`** is a `224 bits long unsigned integer`. A `root` `hash` equals to a `content-hash`.
- **`main-tree`**. It's a tree of subtrees. Each node of the tree is a `subtree`.
- **`sub-tree`**. It's a binary tree of `CDT-node-id`.
- **`node-id`** is a `256 bit long unsigned integer`.
- **`node-height`** is a tuple of two numbers `[main, sub]`:
  - **`main`** is a height of a node in a main tree and it's a `byte`. `0` is a leaf layer. Each subtree adds `1` to the main height.
  - **`sub`** is a height of a node in a subtree and it's a `byte`.
