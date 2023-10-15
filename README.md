# Data Block Set

The `blockset` application is a command line program that can store and retrieve data blocks using a content-dependent tree (CDT) hash function as a universal address of the blocks.

Articles:
- [BLOCKSET v0.2](https://medium.com/@sergeyshandar/blockset-v0-2-b43c03bac182),
- [Content Dependent-Tree](https://medium.com/@sergeyshandar/content-dependent-hash-tree-9e0f60859415).

Install the latest stable version from [crates.io](https://crates.io/crates/blockset):

```console
cargo install blockset
```

Install the current version from the `main` branch:

```console
cargo install --git https://github.com/datablockset/blockset
```

Uninstall the `blockset`:

```console
cargo uninstall blockset
```

### Commands

- address validation:
  ```console
  blockset validate 3v1d4j94scaseqgcyzr0ha5dxa9rx6ppnfbndck971ack
  ```
- calculate address:
  ```console
  blockset address ./README.md
  ```
- add a data block to the local storage `cdt0/`:
  ```console
  blockset add ./LICENSE
  ```
- get a file by address
  ```console
  blockset get ngd7zembwj6f2tsh4gyxrcyx26h221e3f2wdgfbtq87nd ./old.md
  ```

## For Developers

Internal documentation: https://blockset.pages.dev/.

### Best practices

- Make it simple.
- Avoid `unsafe` code. Currently, we don't have `unsafe` code.
- No I/O is allowed in a library. We have 100% code coverage.
- Make `const` functions if possible.
- Avoid using macros. Allowed macros: `derive`, `cfg`, `test`, `assert..`, `wasm_bindgen_test`.
- Avoid using third-party dependencies, especially if they use I/O directly.
