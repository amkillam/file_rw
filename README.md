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
file_rw = "0.3.5"
```

- Run the following Cargo command to automatically do so:

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

```rust
use file_rw::{FileReader, FileWriter};
use tempfile::tempdir;

let tempdir = tempdir().unwrap();
let tempdir_path = tempdir.path();
let test_path = tempdir_path.join("test.txt");
let mut writer = FileWriter::open(&test_path).unwrap();
writer.append("Hello World!"); //Hello World!
writer.overwrite("Hello"); //Hello
writer.write("Hullo"); //Hullo

writer.find_replace_nth("l", "y", 0).unwrap(); //Huylo
writer.find_replace("u", "e").unwrap(); //Heylo
writer.find_replace("lo", "yyy").unwrap(); //Heyyyy
writer.find_replace_all("y", "i").unwrap(); //Heiiii
writer.find_replace("e", "i").unwrap(); //Hiiiii
let reader = writer.to_reader().unwrap();
let content = reader.read_to_string();
assert_eq!(content, "Hiiiii");
```
