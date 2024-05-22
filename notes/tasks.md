# Tasks

- [ ] Fix `file already exists` when two `blockset` processes are working on the same repository.
- [ ] Fix `check` and add unit tests.

- [x] Fix `New 0 MB.` when adding directories.
- [x] Fix progress when extracting directories.

## Epic Stories

### Digital Signatures And Time Stamping

- [ ] Digital Signatures.
- [ ] Time Stamps.

### Add and extract directories

Adding and extracting directories as a single JSON file. For example

```json
{
    "a": "data:...",
    "b": "data:...",
}
```

See:
- https://datatracker.ietf.org/doc/html/rfc2397,
  Note: If <mediatype> is omitted, it defaults to text/plain;charset=US-ASCII.
- https://en.wikipedia.org/wiki/Data_URI_scheme,
- https://developer.mozilla.org/en-US/docs/web/http/basics_of_http/data_urls.

### New Library Structure

- `IO-trait` is a trait with a minimal set of I/O operations.
- `common` a library with common functions, including I/O extensions. Depends on
  - `IO-trait` to extend the `IO` trait.
- `IO-impl` is an implementation of `IO` for the local file system. Depends on
  - `IO-trait`.
  - `common`.
- `IO-test` is a mock implementation of `IO` for testing. Depends on
  - `IO-trait`.
  - `common`.

### ALIQ Model
