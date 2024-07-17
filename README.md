# file_rw

`file_rw` is a Rust crate for high-performance, memory-mapped file I/O utilities.

[![Crates.io][crates-badge]][crates-url]
[![GNU GPLv3 licensed][gpl-badge]][gpl-url]
[![Build Status][actions-badge]][actions-url]
[![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/file_rw.svg
[crates-url]: https://crates.io/crates/file_rw
[gpl-badge]: https://img.shields.io/badge/License-GPLv3-blue.svg
[gpl-url]: https://github.com/amkillam/file_rw/blob/master/LICENSE
[actions-badge]: https://github.com/amkillam/file_rw/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/amkillam/file_rw/actions/workflows/ci.yml
[docs-badge]: https://docs.rs/file_rw/badge.svg
[docs-url]: https://docs.rs/file_rw

## Features

- High-performance file reading and writing capabilities
- Memory-mapped files for efficient access and manipulation
- High-level, efficient abstractions of common operations on file contents

## Installation

You can include the crate in your Rust project by either:

- Adding the following to your `Cargo.toml` file:

```toml
[dependencies]
file_rw = "0.6.1"
```

- Running the following Cargo command to automatically do so:

```bash
cargo add file_rw
```

## Modules

- `file`: File operations
- `read`: File reading capabilities
- `write`: File writing capabilities

## Re-exports

The crate re-exports the `FileReader` and `FileWriter` structs for external use. These structs contain the aforementioned utilities.

## Examples

The following are examples of using methods from the `FileReader` and `FileWriter` structs.
The examples are separated based on the crate features required to run them.
### Simple Read and Write

```rust
use file_rw::{FileReader, FileWriter};
use tempfile::tempdir;

let tempdir = tempdir().unwrap();
let tempdir_path = tempdir.path();
let test_path = tempdir_path.join("test.txt");
let mut writer = FileWriter::open(&test_path).unwrap();

writer.append("Hello World!");
assert_eq!(writer.bytes(), b"Hello World!");

writer.overwrite("Hello");
assert_eq!(writer.bytes(), b"Hello");

writer.write("Hullo");
assert_eq!(writer.bytes(), b"Hullo");
```

### Search and Replace
Use the `search` feature to enable search and replace capabilities.
```rust
use file_rw::{FileReader, FileWriter};
use tempfile::tempdir;

let tempdir = tempdir().unwrap();
let tempdir_path = tempdir.path();
let test_path = tempdir_path.join("test.txt");
let mut writer = FileWriter::open(&test_path).unwrap();
writer.overwrite("Hullo");

#[cfg(feature = "search")]
{
writer.find_replace_nth("l", "y", 0).unwrap();
assert_eq!(writer.bytes(), b"Huylo");

writer.find_replace("u", "e").unwrap();
assert_eq!(writer.bytes(), b"Heylo");

writer.find_replace("lo", "yyy").unwrap();
assert_eq!(writer.bytes(), b"Heyyyy");

writer.find_replace_all("y", "i").unwrap();
assert_eq!(writer.bytes(), b"Heiiii");

writer.find_replace("e", "i").unwrap();
assert_eq!(writer.bytes(), b"Hiiiii");

let reader = writer.as_reader().unwrap();
let content = reader.read_to_string();
assert_eq!(content, "Hiiiii");
}
```

### Hashing
Use the `hash` feature to enable hash capabilities - these methods require providing a
`Digest` to hash with.
```rust
use file_rw::{FileReader, FileWriter};
use tempfile::tempdir;

let tempdir = tempdir().unwrap();
let tempdir_path = tempdir.path();
let test_path = tempdir_path.join("test.txt");
let mut writer = FileWriter::open(&test_path).unwrap();

writer.overwrite("Hello World!");
let reader = writer.as_reader().unwrap();

#[cfg(feature = "hash")]
{
  assert_eq!(reader.hash_to_string_with::<sha3::Sha3_256>(), "d0e47486bbf4c16acac26f8b653592973c1362909f90262877089f9c8a4536af");
  
  use sha3::Digest;
  let mut sha3_direct_hasher = sha3::Sha3_256::new();
  sha3_direct_hasher.update(b"Hello World!");
  assert_eq!(reader.hash_with::<sha3::Sha3_256>(), sha3_direct_hasher.finalize());

}
```

### SHA3_256 Hashing
Use the `sha3_256` feature to enable SHA3_256 hash capabilities - this also enables the
`hash` feature, but provides convenience methods that don't require manually providing a `Digest`.
```rust
use file_rw::{FileReader, FileWriter};
use tempfile::tempdir;

let tempdir = tempdir().unwrap();
let tempdir_path = tempdir.path();
let test_path = tempdir_path.join("test.txt");
let mut writer = FileWriter::open(&test_path).unwrap();

writer.overwrite("Hello World!");
let reader = writer.as_reader().unwrap();

#[cfg(feature = "sha3_256")]
{
  assert_eq!(reader.hash_to_string(), "d0e47486bbf4c16acac26f8b653592973c1362909f90262877089f9c8a4536af");
  
  use sha3::Digest;
  let mut sha3_direct_hasher = sha3::Sha3_256::new();
  sha3_direct_hasher.update(b"Hello World!");
  assert_eq!(reader.hash(), sha3_direct_hasher.finalize());
}
```
