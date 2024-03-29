There are multiple known platforms problems, mostly related to Windows:

- `\` in Windows paths instead of `/`.
- `CRLF` (`\r\n`) line endings instead of `LF` (`\n`).

## Proposed Solution

A layer of abstraction when the BLOCKSET and other programs communicate using the layer. The layer can be configured to handle the platform-specific problems.

### EOL

1. When a program reads a file, the virtual IO layer may detect text files and either
  1. do nothing or
  2. convert `CRLF` to `LF`.
2. When a program writes a file, the virtual IO layer may detect text files and either
  1. do nothing or
  2. convert `LF` to `CRLF`.

Default behavior should be 1.1 and 2.1.

A special tool which can analyze files, warn if there are `CRLF` and fix if required. Output:
- `non-UTF8 (most likely binary)`
- `UTF8`
- `UTF8 with CRLF`

Plan:
1. create a separate tool,
2. remove EOL handling from the BLOCKSET.
