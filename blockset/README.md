# Data Block Set

Installation:

```console
cargo install blockset
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