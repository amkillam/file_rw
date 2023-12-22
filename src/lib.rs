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
//! ## Traits
//!
//! Additionally, it defines the following traits:
//! - `PathRef`: Represents a reference to a file path
//! - `BytesRef`: Represents a reference to a byte array
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
#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use std::path::Path;

mod file;
mod read;
mod write;

pub use read::FileReader;
pub use write::FileWriter;

pub trait PathRef = AsRef<Path> + std::marker::Sync + std::marker::Send;
pub trait BytesRef = AsRef<[u8]> + std::marker::Sync + std::marker::Send;
