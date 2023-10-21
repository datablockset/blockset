# CDT0

An item is a 256bit unsigned integer, `U256`.

A tag is eight highest bits, `item >> 248`:
- `0..=248`: a byte sequence. In this case, a tag is the length of the sequence.
- `255`: a hash.

## From byte to item

`(b as U256) | (0x01 << 248)`
