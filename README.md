# file_rw

`file_rw` is a Rust crate for high-performance, memory-mapped file I/O utilities.

## Features

- High-performance file reading and writing capabilities
- Memory-mapped files for efficient access and manipulation
- Support for opening files for reading, writing, and appending

## Installation

You can include the crate in your Rust project by either:

- Adding the following to your `Cargo.toml` file:

```toml
[dependencies]
file_rw = "0.1.2"
```

- Run the following Cargo command to automatically do so:

```bash
cargo add file_rw
```

## Modules

- `file`: File operations
- `read`: File reading capabilities
- `write`: File writing capabilities

## Reexports

The library reexports the `FileReader` and `FileWriter` structs for external use. These structs contain the aforementioned utilities.

## Examples

```rust
use file_rw::FileReader;

let reader = FileReader::open("example.txt");
let content = reader.read_to_string();
println!("File content: {}", content);
```
