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
//! use file_rw::{FileReader, FileWriter, preprocess::ContinuousHashmap};
//! use tempfile::tempdir;
//!
//! let tempdir = tempdir().unwrap();
//! let tempdir_path = tempdir.path();
//! let test_path = tempdir_path.join("test.txt");
//! let mut writer = FileWriter::open(&test_path);
//! writer.append(&"Hello World!"); //Hello World!
//! writer.overwrite(&"Hello"); //Hello
//! writer.write(&"Hullo"); //Hullo
//!
//! let mut preprocess_cache = writer.preprocess_with::<ContinuousHashmap>();
//! writer.find_replace_nth("l", "y", 0, &mut preprocess_cache); //Huylo
//! writer.find_replace("u", "e", &mut preprocess_cache); //Heylo
//! writer.find_replace("lo", "yyy", &mut preprocess_cache); //Heyyyy
//! let mut preprocess_cache = writer.preprocess();
//! writer.find_replace_all("y", "i", &mut preprocess_cache); //Heiiii
//! writer.find_replace("e", "i", &mut preprocess_cache); //Hiiiii
//! let reader = FileReader::open(&test_path);
//! let content = reader.read_to_string();
//! assert_eq!(content, "Hiiiii");
//! ```

#![crate_name = "file_rw"]
#![crate_type = "lib"]

pub mod file; //mainly pub for use in tests
mod read;
mod write;

pub use read::preprocess;
pub use read::FileReader;
pub use write::FileWriter;
