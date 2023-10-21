# CDT0

Each tree node contains a digest.

A digest is a 256-bit unsigned integer, `U256`.

A tag is eight highest bits, `item >> 248`:
- `0..=248`: a bit sequence. In this case, a tag is the length of the sequence.
- `255`: a hash.

```rust
const MAX_LEN: u32 = 248;

// a length of the sequence in bits.
// The function returns 255 if the digest is a hash.
fn len(a: U256) -> u32 {
    (a >> 248) as u32
}

fn hash_to_digest(hash: U224) -> U256 {
     (hash as U256) | (0xFF << 248)   
}
```

## From byte to digest

```rust
fn byte_to_digest(b: u8) -> U256 {
    (b as U256) | (0x08 << 248)
}
```

## Merge function

```rust
fn get_data(a: U256) -> U256 {
    a & ((1 << 248) - 1)
}

fn merge(a: U256, b: U256) -> U256 {
    let len_a = len(a);
    let len_b = len(b);
    let len = len_a + len_b;
    if len <= MAX_LEN {
        
    } else {
        
    }
}
```
