# BLOCKSET

The `blockset` application is a command line program that can store and retrieve data blocks using a content-dependent tree (CDT) hash function as a universal address of the blocks.

Articles:
- [BLOCKSET v0.6](https://medium.com/@sergeyshandar/blockset-0-6-working-with-directories-and-sync-by-copy-9c25bd52d3cb?sk=d39a14d4804e4e8308f6b81eced68ab9),
- [Content Dependent Tree](https://medium.com/@sergeyshandar/content-dependent-hash-tree-9e0f60859415?sk=b8fd2af2979fc8dd3b58ab024d589057).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install).
- For Windows, you may need Visual C++. You can get either
  - by installing [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/),
  - or adding [Desktop development with C++](https://learn.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=msvc-170) to Visual Studio.

## Installation

To install the latest stable version from [crates.io](https://crates.io/crates/blockset), run:

```console
cargo install blockset
```

To install the current version from the `main` branch, run:

```console
cargo install --git https://github.com/datablockset/blockset
```

To unininstall the `blockset`, run:

```console
cargo uninstall blockset
```

[Command line interface](./blockset/README.md#commands).

## For Developers

Internal documentation: https://blockset.pages.dev/.

### Best practices

- Make it simple.
- Avoid `unsafe` code. Currently, we don't have `unsafe` code.
- No I/O is allowed in a library. We have 100% code coverage.
- Make `const` functions if possible.
- Avoid using macros. Allowed macros: `derive`, `cfg`, `test`, `assert..`, `wasm_bindgen_test`.
- Avoid using third-party dependencies, especially if they use I/O directly.
