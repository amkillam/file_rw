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
//! use file_rw::{FileReader, FileWriter};
//! use tempfile::tempdir;
//!
//! let tempdir = tempdir().unwrap();
//! let tempdir_path = tempdir.path();
//! let test_path = tempdir_path.join("test.txt");
//! let mut writer = FileWriter::open(&test_path);
//! writer.append(&"Hello World!"); //Hello World!
//! writer.overwrite(&"Hello"); //Hello
//! writer.write(&"Hullo"); //Hullo
//! writer.find_replace_nth(&"l", &"r", 1); //Hulro
//! writer.find_replace(&"o", &"ooooooo"); //Hulrooooooo
//! writer.find_replace_all(&"o", &"d"); //Hulrddddddd
//! let reader = FileReader::open(&test_path);
//! let content = reader.read_to_string();
//! assert_eq!(content, "Hulrddddddd");
//! ```

#![crate_name = "file_rw"]
#![crate_type = "lib"]

pub mod file; //mainly pub for use in tests
mod read;
mod write;

pub use read::FileReader;
pub use write::FileWriter;
