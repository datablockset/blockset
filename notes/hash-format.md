# CDT0

An item is a 256bit unsigned integer, `U256`.

A tag is eight highest bits, `item >> 248`:
- `[0; 248]`: a byte sequence.
- `255`: a hash.
