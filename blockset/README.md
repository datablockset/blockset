# BLOCKSET

The `blockset` application is a command line program that can store and retrieve data blocks using a content-dependent tree (CDT) hash function as a universal address of the blocks.

## Commands

- content hash validation:
  ```console
  blockset validate 3v1d4j94scaseqgcyzr0ha5dxa9rx6ppnfbndck971ack
  ```
- calculate a content hash of a file:
  ```console
  blockset hash ./README.md
  ```
- add content of a file or a directory to the local storage `cdt0/`:
  ```console
  blockset add ./README.md
  blockset add ./src/ --to-posix-eol
  ```
- get a file or a directory by a content hash
  ```console
  blockset get ngd7zembwj6f2tsh4gyxrcyx26h221e3f2wdgfbtq87nd ./ls.json
  blockset get ngd7zembwj6f2tsh4gyxrcyx26h221e3f2wdgfbtq87nd ./dir/
  ```
- information about the repository
  ```console
  blockset info
  ```
