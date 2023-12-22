
# file_rw

`file_rw` is a Rust library for high-performance, memory-mapped file reading and writing utilities.
## Features

- High-performance file reading and writing capabilities
- Memory-mapped files for efficient access and manipulation
- Support for opening files for reading, writing, and appending

## Installation

You can include the library in your Rust project by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
file_rw = "0.1.0"
```

## Modules
- `file`: File operations
- `read`: File reading capabilities
- `write`: File writing capabilities

## Reexports
The library reexports the `FileReader` and `FileWriter` structs for external use.

## Traits
Additionally, it defines the following traits:
- `PathRef`: Represents a reference to a file path
- `BytesRef`: Represents a reference to a byte array

## Examples
```rust
use file_rw::FileReader;

let reader = FileReader::open("example.txt");
let content = reader.read_to_string();
println!("File content: {}", content);
```
