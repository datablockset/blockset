# File Format

All files are stored in one directory `cdt0/`. CDT stands for "Content Dependent Tree". `0` means a revision of the hash function.

## File Types

Two types of files:

- a part of a data block. The file name starts with `_` and is followed by the base32 encoded `hash` of the data block.
- whole data block. The file name is a base32 encoded `root` of the data block. A `root` is `compress([hash, empty])`.

## File Subtypes

The first byte:

- `0..=0x1F` is the length of a tail
  - tail,
  - a list of nodes.
- `0x20`: data.
- `0x21..` are reserved for different things, such as subtrees and compressed data.

## Data Levels

|Level|min,B  |x3,B      |
|-----|-------|----------|
|1    |2      |3         |
|2    |4      |9         |
|3    |8      |27        |
|4    |16     |81        |
|5    |32     |243       |
|6    |64     |729       |
|7    |128    |2_187     |
|**8**|**256**|**6_561** |
|9    |512    |19_683    |
|10   |1_024  |59_049    |
|11   |2_048  |177_147   |
|12   |4_096  |531_441   |
|13   |8_192  |1_594_323 |
|14   |16_384 |4_782_969 |
|15   |32_768 |14_348_907|
|16   |65_536 |43_046_721|

## Node Levels

|Level|min   |x3    |min,B  |x3,B     |
|-----|------|------|-------|---------|
|1    |2     |3     |56     |84       |
|2    |4     |9     |112    |252      |
|3    |8     |27    |224    |756      |
|**4**|**16**|**81**|**448**|**2_268**|
|5    |32    |243   |896    |6_804    |
|6    |64    |729   |1_792  |20_412   |
|7    |128   |2_187 |3_584  |61_236   |
|8    |256   |6_561 |7_168  |183_708  |
