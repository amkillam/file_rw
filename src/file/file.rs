use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn open_as_write(path: &Path) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}

pub fn open_as_read(path: &Path) -> File {
    OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}

pub fn open_as_append(path: &Path) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}
