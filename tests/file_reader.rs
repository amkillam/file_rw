use file_rw::FileReader;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha3::{Digest, Sha3_256};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[cfg(test)]

fn create_test_files() {
    let mut file = File::create("tests/test_file").unwrap();
    file.write(b"test file\n").unwrap();

    let mut file = File::create("tests/test_file2").unwrap();
    file.write(b"test file\n").unwrap();

    let mut diff_file = File::create("tests/test_file_diff").unwrap();
    diff_file.write(b"test file diff\n").unwrap();

    let mut file = File::create("tests/test_find_bytes").unwrap();
    file.write(b"        test test test test\n").unwrap();
}

fn destroy_test_files() {
    std::fs::remove_file("tests/test_file").unwrap();
    std::fs::remove_file("tests/test_find_bytes").unwrap();
}

#[test]
fn test_open() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    assert_eq!(file_reader.read_to_string(), "test file\n");
    destroy_test_files();
}

#[test]
fn test_open_file() {
    create_test_files();
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("tests/test_file")
        .unwrap();
    let file_reader = FileReader::open_file(&file);
    assert_eq!(file_reader.read_to_string(), "test file\n");
    destroy_test_files();
}

#[test]
fn test_read_to_string() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    assert_eq!(file_reader.read_to_string(), "test file\n");
    destroy_test_files();
}

#[test]
fn test_bytes() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    assert_eq!(file_reader.bytes(), b"test file\n");
    destroy_test_files();
}

#[test]
fn test_to_vec() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    assert_eq!(file_reader.to_vec(), b"test file\n");
    destroy_test_files();
}

#[test]
fn test_file() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let file = file_reader.file();
    assert_eq!(file_reader.read_to_string(), "test file\n");
    destroy_test_files();
}

#[test]
fn test_mmap() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    assert_eq!(file_reader.mmap().len(), 10);
    destroy_test_files();
}

#[test]
fn test_path() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    assert_eq!(file_reader.path(), Path::new("tests/test_file"));
    destroy_test_files();
}

#[test]
fn test_to_writer() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let mut file_writer = file_reader.to_writer();
    file_writer.write(b"test file\n");
    assert_eq!(file_reader.read_to_string(), "test file\ntest file\n");
    destroy_test_files();
}

#[test]
fn test_hash() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let expected_hash = sha3::Sha3_256::digest(b"test file\n");
    assert_eq!(file_reader.hash(), expected_hash);
    destroy_test_files();
}

#[test]
fn test_hash_with() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let hash_fn_output = file_reader.hash();
    let hash_with_fn_output = file_reader.hash_with::<sha3::Sha3_256>();
    assert_eq!(hash_fn_output, hash_with_fn_output);
    destroy_test_files();
}

#[test]
fn test_hash_to_string() {
    create_test_files();
    let expected_string = "301285a36e29434a5a13a2c307284e0d64edf827b723290ff4351a6414aa18cf"
        .to_uppercase()
        .to_string();
    let file_reader = FileReader::open("tests/test_file");
    assert_eq!(file_reader.hash_to_string(), expected_string);
    destroy_test_files();
}

#[test]
fn test_find_bytes() {
    create_test_files();
    let file = FileReader::open("tests/test_find_bytes");
    let bytes = b"test";
    let expected_offset = 9;

    assert_eq!(file.find_bytes(bytes), Some(expected_offset));
    destroy_test_files();
}

#[test]
fn test_rfind_bytes() {
    create_test_files();
    let file = FileReader::open("tests/test_find_bytes");
    let bytes = b"test";
    let expected_offset = 0;

    assert_eq!(file.rfind_bytes(bytes), Some(expected_offset));
    destroy_test_files();
}

#[test]
fn test_find_bytes_all() {
    create_test_files();
    let file = FileReader::open("tests/test_find_bytes");
    let bytes = b"test";
    let expected_offsets = vec![9, 13, 17];

    assert_eq!(file.find_bytes_all(bytes), expected_offsets);
    destroy_test_files();
}

#[test]
fn test_find_bytes_nth() {
    create_test_files();
    let file = FileReader::open("tests/test_find_bytes");
    let bytes = b"test";
    let expected_offset = 13;

    assert_eq!(file.find_bytes_nth(bytes, 1), Some(expected_offset));
    destroy_test_files();
}

#[test]
fn test_compare_files() {
    create_test_files();
    assert!(FileReader::compare_files("tests/test_file", "test_file2"));
    assert!(!FileReader::compare_files(
        "tests/test_file",
        "test_file_diff"
    ));
    destroy_test_files();
}

#[test]
fn test_compare_to() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    assert!(file_reader.compare_to("tests/test_file2"));
    assert!(!file_reader.compare_to("tests/test_file_diff"));
    destroy_test_files();
}

#[test]
fn test_compare_to_file() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("tests/test_file2")
        .unwrap();
    assert!(file_reader.compare_to_file(&file));
    destroy_test_files();
}

#[test]
fn test_compare_hash() {
    create_test_files();

    let file_reader = FileReader::open("tests/test_file");
    let self_hash = file_reader.hash();

    let file_reader_same = FileReader::open("tests/test_file2");
    let same_hash = file_reader_same.hash();

    let file_reader_diff = FileReader::open("tests/test_file_diff");
    let diff_hash = file_reader_diff.hash();

    assert!(file_reader.compare_hash::<Sha3_256>(&self_hash));
    assert!(file_reader.compare_hash::<Sha3_256>(&same_hash));
    assert!(!file_reader.compare_hash::<Sha3_256>(&diff_hash));

    destroy_test_files();
}

#[test]
fn test_into_iter() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let mut iter = file_reader.into_iter();
    assert_eq!(iter.next(), Some(b't'));
    assert_eq!(iter.next(), Some(b'e'));
    assert_eq!(iter.next(), Some(b's'));
    assert_eq!(iter.next(), Some(b't'));
    assert_eq!(iter.next(), Some(b' '));
    assert_eq!(iter.next(), Some(b'f'));
    assert_eq!(iter.next(), Some(b'i'));
    assert_eq!(iter.next(), Some(b'l'));
    assert_eq!(iter.next(), Some(b'e'));
    assert_eq!(iter.next(), Some(b'\n'));
    assert_eq!(iter.next(), None);
    destroy_test_files();
}

#[test]
fn test_into_par_iter() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let par_iter = file_reader.into_par_iter();
    assert!(par_iter.clone().any(|c| c == b't'));
    assert!(par_iter.clone().any(|c| c == b'e'));
    assert!(par_iter.clone().any(|c| c == b's'));
    assert!(par_iter.clone().any(|c| c == b't'));
    assert!(par_iter.clone().any(|c| c == b' '));
    assert!(par_iter.clone().any(|c| c == b'f'));
    assert!(par_iter.clone().any(|c| c == b'i'));
    assert!(par_iter.clone().any(|c| c == b'l'));
    assert!(par_iter.clone().any(|c| c == b'e'));
    assert!(par_iter.clone().any(|c| c == b'\n'));
    destroy_test_files();
}

#[test]
fn test_eq() {
    create_test_files();
    let file_reader = FileReader::open("tests/test_file");
    let file_reader_same = FileReader::open("tests/test_file2");
    let file_reader_diff = FileReader::open("tests/test_file_diff");
    assert_eq!(file_reader, file_reader_same);
    assert_ne!(file_reader, file_reader_diff);
    destroy_test_files();
}
