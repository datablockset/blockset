# Glossary

- **`content-id`**, **`CID`** is a hash of a content, a data block. A `content-id`` is in `base32` format.
- **`content-dependent-tree`**, **`CD-tree`**, **`CDT`**. The CDT nodes can be split as a `storage-tree` or `main-tree`.
- **`CDT0`** is a name of a hash function which is based on a `content-dependent tree` and the `SHA-224` compression function.
- **`storage-tree`**, **`storage-tree`** is a tree in a storage.  In particular, a `blockset` storage keeps its nodes in files. One node is one file. Each storage may have its own size limitation for one node, however, the split algorithm should aligned levels and sublevels to `2^n`. In this case, a storage with smaller limit will always have blocks for a storage with bigger limit.
- **`storage-node-id`** is a hash of a node of data. Contains
    - **`type`** is an either `root` or `child`.
    - **`hash`** is a `224 bits long unsigned integer`. A `root` `hash` equals to a `content-id`.
- **`main-tree`**. It's a tree of subtrees. Each node of the tree is a `CDT` subtree.
- **`sub-tree`**. It's a binary tree of `CDT-node-id`.
- **`node-id`** is a `256 bit long unsigned integer`.
- **`node-level`** is two numbers:
  - **`node-main-level`** is a level of the main tree and it's `byte`. `0` is a leaf level. Each subtree adds `1` to the level.
  - **`node-sublevel`** is a level of a subtree and it's `byte`.
