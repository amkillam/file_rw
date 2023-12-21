use std::fs::{OpenOptions, File};
use std::path::Path;

pub fn open_as_write<PathRef: AsRef<Path>>(path: PathRef) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}

pub fn open_as_read<PathRef: AsRef<Path>>(path: PathRef) -> File {
    OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}

pub fn open_as_append<PathRef: AsRef<Path>>(path: PathRef) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Could not open file. Error: {}", err))
}