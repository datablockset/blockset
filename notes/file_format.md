# File Format

All files are stored in one directory `cdt0/`. CDT stands for "Content Dependent Tree". `0` means a revision of the hash function.

## File Types

Two types of files:

- a part of a data block. The file name starts with `_` and followed by the base32 encoded `hash` of the data block.
- whole data block. The file name is a base32 encoded `final_hash` of the data block. A `final_hash` is `compress([hash, empty])`.

## File Subtypes

The first byte:

- `0x00`: data
- `0x01`: a list of hashes of the same level, each hash is 28 bytes long.
- `0x02`: same as `0x01`, but it has a data block (less than 32 bytes) at the beginning. The data block should be attached to the end of the list of hashes. It's only used if a tail data block is less than 32 bytes and can't be saved in a separate data file `0x00`.