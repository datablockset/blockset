# Design

## Dependencies

```mermaid
graph TD
  io-trait
  io-impl & io-test --> io-trait
  io-impl --> libc
  blockset-lib-tests[blockset-lib\nunit tests] --> io-test & blockset-lib
  blockset --> blockset-lib --> io-trait
  blockset --> io-impl
```
