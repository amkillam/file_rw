use file_rw::file::{open_as_read, open_as_write};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

#[cfg(test)]
#[test]
fn test_open_as_write() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let path = tempdir_path.join("test_open_as_write");
    let path = PathBuf::from(&path);
    let mut file = open_as_write(&path);
    file.write_all(b"test").unwrap();
    assert!(path.exists());
    assert!(file.metadata().unwrap().len() > 0);
}

#[test]
fn test_open_as_read() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let path = tempdir_path.join("test_open_as_read");
    let mut file = open_as_write(&path);
    file.write_all(b"test").unwrap();
    assert!(path.exists());
    let file = open_as_read(&path);
    assert!(file.metadata().unwrap().len() > 0);
}
