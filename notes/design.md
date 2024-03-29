# Design

## Dependencies

```mermaid
graph TD
  io-trait
  io-impl & io-test --> io-trait
  io-impl --> libc
  blockset-lib-test[blockset-lib\nunit tests] --> wasm-bindgen-test & io-test & blockset-lib
  blockset-lib --> nanvm-lib --> io-trait
  blockset --> blockset-lib & io-impl
```

```mermaid
flowchart TD
  subgraph br [blockset repository]
    blockset-lib-test[blockset-lib\ntests]
    blockset-lib
    blockset
  end
  subgraph nanvm [nanvm repository]
    nanvm-lib
  end
  subgraph ior [io repository]
    io-test
    io-trait
    io-impl
  end
  subgraph tpl [third-party libraries]
    wasm-bindgen-test
    libc
  end
  blockset-lib-test --> wasm-bindgen-test & io-test
  blockset-lib --> nanvm-lib --> io-trait
  blockset --> io-impl
  io-impl --> libc
```
