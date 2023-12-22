use file_rw::{FileReader, FileWriter};
use std::fs::{remove_file, OpenOptions};

#[cfg(test)]
#[test]

fn test_open_file() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tests/test_open_file")
        .unwrap();
    let mut file_writer = FileWriter::open_file(file);
    file_writer.write(&"Hello, world!".as_bytes());
    let file_reader = FileReader::open("tests/test_open_file");
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_open_file").unwrap();
}

#[test]
fn test_open() {
    let mut file_writer = FileWriter::open("tests/test_open");
    file_writer.write(&"Hello, world!".as_bytes());
    let file_reader = FileReader::open("tests/test_open");
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_open").unwrap();
}

#[test]
fn test_open_append() {
    let mut file_writer = FileWriter::open_append("tests/test_open_append");
    file_writer.write(&"Hello, world!".as_bytes());
    let file_reader = FileReader::open("tests/test_open_append");
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_open_append").unwrap();
}

#[test]
fn test_write() {
    let mut file_writer = FileWriter::open("tests/test_write");
    file_writer.write(&"Hello, world!".as_bytes());
    let file_reader = FileReader::open("tests/test_write");
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_write").unwrap();
}

#[test]
fn test_replace() {
    let mut file_writer = FileWriter::open("tests/test_replace");
    file_writer.write(&"Hello, world!".as_bytes());
    file_writer.replace(&"Hello".as_bytes(), 0);
    let file_reader = FileReader::open("tests/test_replace");
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_replace").unwrap();
}

#[test]
fn test_find_replace() {
    let mut file_writer = FileWriter::open("tests/test_find_replace");
    file_writer.write(&"Hello, world!".as_bytes());
    file_writer.find_replace(&"Hello".as_bytes(), &"Goodbye".as_bytes());
    let file_reader = FileReader::open("tests/test_find_replace");
    assert_eq!(file_reader.read_to_string(), "Goodbye, world!");
    remove_file("tests/test_find_replace").unwrap();
}

#[test]
fn test_find_replace_nth() {
    let mut file_writer = FileWriter::open("tests/test_find_replace_nth");
    file_writer.write(&"Hello, world!".as_bytes());
    file_writer.find_replace_nth(&"o".as_bytes(), &"a".as_bytes(), 2);
    let file_reader = FileReader::open("tests/test_find_replace_nth");
    assert_eq!(file_reader.read_to_string(), "Hello, warld!");
    remove_file("tests/test_find_replace_nth").unwrap();
}

#[test]
fn test_find_replace_all() {
    let mut file_writer = FileWriter::open("tests/test_find_replace_all");
    file_writer.write(&"Hello, world!".as_bytes());
    file_writer.find_replace_all(&"o".as_bytes(), &"a".as_bytes());
    let file_reader = FileReader::open("tests/test_find_replace_all");
    assert_eq!(file_reader.read_to_string(), "Hella, warld!");
    remove_file("tests/test_find_replace_all").unwrap();
}

#[test]
fn test_file() {
    let mut file_writer = FileWriter::open("tests/test_file");
    file_writer.write(&"Hello, world!".as_bytes());
    let file = file_writer.file();
    let file_reader = FileReader::open_file(&file);
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_file").unwrap();
}

#[test]
fn test_path() {
    let mut file_writer = FileWriter::open("tests/test_path");
    file_writer.write(&"Hello, world!".as_bytes());
    let path = file_writer.path();
    let file_reader = FileReader::open(path.as_ref());
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_path").unwrap();
}

#[test]
fn test_mmap() {
    let mut file_writer = FileWriter::open("tests/test_mmap");
    file_writer.write(&"Hello, world!".as_bytes());
    let file_reader = FileReader::open("tests/test_mmap");
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_mmap").unwrap();
}

#[test]
fn test_to_reader() {
    let mut file_writer = FileWriter::open("tests/test_to_reader");
    file_writer.write(&"Hello, world!".as_bytes());
    let file_reader = file_writer.to_reader();
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    remove_file("tests/test_to_reader").unwrap();
}
