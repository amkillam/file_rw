#[cfg(test)]
use file_rw::{utils::*, FileWriter};
use tempfile::tempdir;

#[test]
fn test_read_to_buf() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_read_to_buf");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = b"hello";
    writer.overwrite(input).unwrap();
    let mut buf = [0; 5];
    let out_len = read_to_buf(&path, &mut buf).unwrap();
    assert_eq!(out_len, input.len());
    assert_eq!(&buf, b"hello");
}

#[test]
fn test_read_to_vec() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_read_to_vec");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = b"hello";
    writer.overwrite(input).unwrap();
    let vec = read_to_vec(&path).unwrap();
    assert_eq!(&vec, b"hello");
}

#[test]
fn test_read_to_string() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_read_to_string");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = "hello";
    writer.overwrite(input).unwrap();
    let string = read_to_string(&path).unwrap();
    assert_eq!(&string, "hello");
}

#[test]
fn test_read_to() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_read_to");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = b"hello";
    writer.overwrite(input).unwrap();
    let vec: Vec<u8> = read_to::<Vec<u8>>(&path).unwrap();
    assert_eq!(vec.as_slice(), b"hello");
}

#[test]
fn test_read_to_writer() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_read_to_writer");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = b"hello";
    writer.overwrite(input).unwrap();
    let mut buf = Vec::new();
    read_to_writer(&path, &mut buf).unwrap();
    assert_eq!(&buf, b"hello");
}

#[test]
fn test_overwrite() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_overwrite");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = b"hello";
    writer.overwrite(input).unwrap();
    let vec = read_to_vec(&path).unwrap();
    assert_eq!(&vec, b"hello");
}

#[test]
fn test_append() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_append");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = b"hello";
    writer.overwrite(input).unwrap();
    let input = b" world";
    writer.append(input).unwrap();
    let vec = read_to_vec(&path).unwrap();
    assert_eq!(&vec, b"hello world");
}

#[test]
fn test_get_mmap_read() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_get_mmap_read");
    let mut writer = FileWriter::open(&path).unwrap();
    let input = b"hello";
    writer.overwrite(input).unwrap();
    let mmap = get_mmap_read(&path).unwrap();
    assert_eq!(&mmap[..], b"hello");
}

#[test]
fn test_get_mmap_write() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_get_mmap_write");
    let len = 5;
    let mut mmap = get_mmap_write(&path, len).unwrap();
    mmap[..].copy_from_slice(b"hello");
    let mmap = get_mmap_read(&path).unwrap();
    assert_eq!(&mmap[..], b"hello");
}
