# Block Types

```ts
// a CDT0 hash.
type Hash = string
```

## Digital Signature

```ts
type SignatureTag = {
  signature: Signature
}
type Signature = {
  public_key: string
  hash: Hash
  signature: string 
}
```

## Revision (Version)

```ts
type RevisionTag = {
  revision: Revision
}
type Revision = {
  previous: Hash[]
  current: Hash
}
```

## Directory

```ts
type DirectoryTag = {
  directory: Directory
}
type Directory = {
  [path in Path]: Hash 
}
// a path using `/` as a separator.
type Path = string
```
