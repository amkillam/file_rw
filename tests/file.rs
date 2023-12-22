use file_rw::file::{open_as_append, open_as_read, open_as_write};
use std::fs::remove_file;
use std::io::Write;
use std::path::PathBuf;

#[cfg(test)]
#[test]
fn test_open_as_write() {
    let path = PathBuf::from("tests/test_open_as_write.txt");
    let mut file = open_as_write(&path);
    file.write_all(b"test").unwrap();
    assert!(path.exists());
    remove_file(path).unwrap();
}

#[test]
fn test_open_as_read() {
    let path = PathBuf::from("tests/test_open_as_read.txt");
    let mut file = open_as_write(&path);
    file.write_all(b"test").unwrap();
    assert!(path.exists());
    let file = open_as_read(&path);
    assert!(file.metadata().unwrap().len() > 0);
    remove_file(path).unwrap();
}

#[test]
fn test_open_as_append() {
    let path = PathBuf::from("tests/test_open_as_append.txt");
    let mut file = open_as_write(&path);
    file.write_all(b"test").unwrap();
    assert!(path.exists());
    let mut file = open_as_append(&path);
    file.write_all(b"test").unwrap();
    assert!(path.exists());
    remove_file(path).unwrap();
}
