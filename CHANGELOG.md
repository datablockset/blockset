# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.5.0

- `blockset get` can extract multiple files to a directory. PR [#169](https://github.com/datablockset/blockset/pull/169)
- `blockset get` can create directories recursively. PR [#168](https://github.com/datablockset/blockset/pull/168).
- `blockset add` works with directories. PR [#165](https://github.com/datablockset/blockset/pull/165).

## 0.4.2

- WASI. PR [#158](https://github.com/datablockset/blockset/pull/158).

## 0.4.1

- Estimated time left. PR [#151](https://github.com/datablockset/blockset/pull/151).

## 0.4.0

- Replace the `blockset address` command to `blockset hash`. PR [#147](https://github.com/datablockset/blockset/pull/147).

## 0.3.7

- Better info estimate. Issue [#138](https://github.com/datablockset/blockset/issues/138).
- Internal: replace `Result<T, String>` to `io::Result<T>`. Issue [#70](https://github.com/datablockset/blockset/issues/70).

## 0.3.6

- Option `--to-posix-eol` for converting Windows line endings to POSIX line endings. Issue [#133](https://github.com/datablockset/blockset/issues/131).

## 0.3.5

- Two packages `blockset` and `blockset-cli`. PR [#121](https://github.com/datablockset/blockset/pull/121).

## 0.3.3

- Another bug fix in `info` (division by zero). PR [#108](https://github.com/datablockset/blockset/pull/108).

## 0.3.2

- An improved info progress message. PR [#104](https://github.com/datablockset/blockset/pull/104).
- Fix for the "Not a directory" info bug. PR [#106](https://github.com/datablockset/blockset/pull/106).

## 0.3.1

- `info` command. PR [#99](https://github.com/datablockset/blockset/pull/99).

## 0.3.0

- Breaking change in the repository directory structure. Issue: [#78](https://github.com/datablockset/blockset/issues/78).

## 0.2.4

- Fix a progress message. PR [#85](https://github.com/datablockset/blockset/pull/85).

## 0.2.3

- Show how much new data is stored. Issue [#79](https://github.com/datablockset/blockset/issues/79).

## 0.2.2

- Show progress in `MB`. Issue [#73](https://github.com/datablockset/blockset/issues/73).
- Show progress during `get`. Issue [#49](https://github.com/datablockset/blockset/issues/49).

## 0.2.1

- Show progress during `add`. Issue [#49](https://github.com/datablockset/blockset/issues/49).

## 0.2.0

The first working version.
