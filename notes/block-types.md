# Block Types

```ts
// a CDT0 hash.
type DataAddress = string
```

## Digital Signature

```ts
type SignatureTag = {
  signature: Signature
}
type Signature = {
  publicKey: string
  dataAddress: DataAddress
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
  [path in Path]: DataAddress 
}
// a path using `/` as a separator.
type Path = string
```
