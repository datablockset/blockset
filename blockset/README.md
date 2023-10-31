# BLOCKSET

The `blockset` application is a command line program that can store and retrieve data blocks using a content-dependent tree (CDT) hash function as a universal address of the blocks.

## Commands

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
  blockset add ./README.md
  blockset add ./LICENSE --to-posix-eol
  ```
- get a file by address
  ```console
  blockset get ngd7zembwj6f2tsh4gyxrcyx26h221e3f2wdgfbtq87nd ./old.md
  ```
- information about the repository
  ```console
  blockset info
  ```