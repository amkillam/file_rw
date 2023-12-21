#![crate_name = "file_rw"]
#![crate_type = "lib"]

#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use std::path::Path;

use digest::{
    core_api::{
        AlgorithmName, BlockSizeUser, BufferKindUser, FixedOutputCore,
        OutputSizeUser, Reset, UpdateCore,
    },
    HashMarker
};

mod read;
mod write;
mod file;

pub use read::FileReader;
pub use write::FileWriter;

pub trait HashFn = HashMarker + BlockSizeUser + BufferKindUser + OutputSizeUser + UpdateCore + FixedOutputCore + Default + Reset + AlgorithmName;
pub trait PathRef = AsRef<Path>;