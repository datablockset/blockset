# File Format

The first byte:

- 0x0: data
- 0x1: a list of hashes of the same level
- 0x2: a final block. It may contain data at the end of the file.