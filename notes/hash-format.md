# CDT0

Each tree node contains a digest.

A digest is a 256-bit unsigned integer, `U256`.

A tag is eight highest bits, `item >> 248`:
- `0..=248`: a byte sequence. In this case, a tag is the length of the sequence.
- `255`: a hash.

## From byte to item

`(b as U256) | (0x01 << 248)`
