# File Format

All files are stored in one directory `cdt0/`. CDT stands for "Content Dependent Tree". `0` means a revision of the hash function.

## File Types

Two types of files:

- a part of a data block. The file name starts with `_` and followed by the base32 encoded `hash` of the data block.
- whole data block. The file name is a base32 encoded `root` of the data block. A `root` is `compress([hash, empty])`.

## File Subtypes

The first byte:

- `0..=0x1F` is a length of a tail
  - tail,
  - a list of nodes
- `0x20`: data