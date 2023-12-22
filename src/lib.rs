#![crate_name = "file_rw"]
#![crate_type = "lib"]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![feature(trivial_bounds)]

use std::path::Path;

mod file;
mod read;
mod write;

pub use read::FileReader;
pub use write::FileWriter;

pub trait PathRef = AsRef<Path> + std::marker::Sync + std::marker::Send;
pub trait BytesRef = AsRef<[u8]> + std::marker::Sync + std::marker::Send;
