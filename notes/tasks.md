# Tasks

- [ ] Fix `New 0 MB.` when adding directories
- [ ] Fix progress when extracting directories

## Epic Stories

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
