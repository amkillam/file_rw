use std::fs::{File, OpenOptions};
use std::path::Path;

/// Opens a file for writing and returns a `File` instance.
pub fn open_as_write(path: &Path) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}

/// Opens a file for reading and returns a `File` instance.
pub fn open_as_read(path: &Path) -> File {
    OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}
