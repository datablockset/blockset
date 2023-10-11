# File Format

The first byte:

- 0x0: data
- 0x1: a list of hashes of the same level, each hash is 28 bytes long.
- 0x2: a final block. It may contain data at the end of the file.

## 0x0. Data

When we read the file, we apply all the bytes but the last one to `tree.push()`. The last byte should be applied to `subtree.end()`. The `end()` function will return the root hash of the subtree.

## 0x1. A list of hashes

When we read the file, we apply all the items but the last one in the list to `tree.push()`. The last item should be applied to `tree.end()`. The `end()` function will return the root hash of the subtree.

## 0x2. A final block
