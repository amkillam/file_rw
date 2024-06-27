use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

/// Opens a file for writing and returns a `File` instance.
#[allow(clippy::suspicious_open_options)] //open option create(true) is on purpose - create iff file does not exist
pub fn open_as_write(path: &Path) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}

/// Opens a file for reading and returns a `File` instance.
pub fn open_as_read(path: &Path) -> io::Result<File> {
    OpenOptions::new().read(true).open(path)
}
