# Data Block Set

Data Block Set

## For Developers

Internal documentation: https://blockset.pages.dev/.

### Testing `blockset` from the `main` branch

Installation:

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
- add to the local storage `cdt0/`:
  ```console
  blockset add ./LICENSE
  ```
- get a fila by address
  ```console
  blockset get ngd7zembwj6f2tsh4gyxrcyx26h221e3f2wdgfbtq87nd ./old.md
  ```
