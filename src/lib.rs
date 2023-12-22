//! # File_RW
//!
//! `file_rw` is a Rust library for efficient file reading and writing.
//!
//! It provides the following modules:
//! - `file`: File operations
//! - `read`: File reading capabilities
//! - `write`: File writing capabilities
//!
//! ## Reexports
//!
//! The library reexports the `FileReader` and `FileWriter` types for external use.
//!
//! ## Examples
//!
//! ```rust
//! use file_rw::FileReader;
//!
//! let reader = FileReader::open("example.txt");
//! let content = reader.read_to_string();
//! println!("File content: {}", content);
//! ```

#![crate_name = "file_rw"]
#![crate_type = "lib"]

pub mod file; //mainly pub for use in tests
mod read;
mod write;

pub use read::FileReader;
pub use write::FileWriter;
