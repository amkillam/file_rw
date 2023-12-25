use file_rw::preprocess::{CharIndexMatrix, ContinuousHashmap, WindowsHashmap};
use file_rw::FileReader;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha3::{Digest, Sha3_256};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;

#[cfg(test)]

fn create_test_files(dir: &Path) {
    let mut file = File::create(dir.join("test_file")).unwrap();
    file.write(b"test file\n").unwrap();

    let mut file2 = File::create(dir.join("test_file2")).unwrap();
    file2.write(b"test file\n").unwrap();

    let mut diff_file = File::create(dir.join("test_file_diff")).unwrap();
    diff_file.write(b"test file diff\n").unwrap();

    let mut find_bytes_file = File::create(dir.join("test_find_bytes")).unwrap();
    find_bytes_file
        .write(b"        test test test test\n")
        .unwrap();
}

#[test]
fn test_open() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert_eq!(file_reader.read_to_string(), "test file\n");
}

#[test]
fn test_open_file() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file = OpenOptions::new()
        .read(true)
        .open(tempdir.path().join("test_file"))
        .unwrap();
    let file_reader = FileReader::open_file(&file);
    assert_eq!(file_reader.read_to_string(), "test file\n");
}

#[test]
fn test_read_to_string() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert_eq!(file_reader.read_to_string(), "test file\n");
}

#[test]
fn test_bytes() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert_eq!(file_reader.bytes(), b"test file\n");
}

#[test]
fn test_to_vec() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert_eq!(file_reader.to_vec(), b"test file\n");
}

#[test]
fn test_file() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert_eq!(file_reader.read_to_string(), "test file\n");
}

#[test]
fn test_mmap() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert_eq!(file_reader.mmap().len(), 10);
}

#[test]
fn test_path() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let path = tempdir.path().join("test_file");
    let file_reader = FileReader::open(&path);
    assert_eq!(file_reader.path(), &path);
}

#[test]
fn test_to_writer() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    let mut file_writer = file_reader.to_writer();
    file_writer.write(b"testwrite\n");
    assert_eq!(file_reader.read_to_string(), "testwrite\n");
}

#[test]
fn test_hash() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    let expected_hash = sha3::Sha3_256::digest(b"test file\n");
    assert_eq!(file_reader.hash(), expected_hash);
}

#[test]
fn test_hash_with() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    let hash_fn_output = file_reader.hash();
    let hash_with_fn_output = file_reader.hash_with::<sha3::Sha3_256>();
    assert_eq!(hash_fn_output, hash_with_fn_output);
}

#[test]
fn test_hash_to_string() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let expected_string =
        "301285a36e29434a5a13a2c307284e0d64edf827b723290ff4351a6414aa18cf".to_string();
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert_eq!(file_reader.hash_to_string(), expected_string);
}

#[test]
fn test_find_bytes() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file = FileReader::open(tempdir.path().join("test_find_bytes"));
    let bytes = b"test";
    let expected_offset = 8;

    let mut preprocess_cache_windows_hashmap = file.preprocess_with::<WindowsHashmap>();
    let mut preprocess_cache_char_index_matrix = file.preprocess_with::<CharIndexMatrix>();
    let mut preprocess_cache_continuous_hashmap = file.preprocess_with::<ContinuousHashmap>();

    let mut preprocess_cache_default = file.preprocess();

    assert_eq!(
        file.find_bytes(bytes, &mut preprocess_cache_windows_hashmap),
        Some(expected_offset)
    );

    assert_eq!(
        file.find_bytes(bytes, &mut preprocess_cache_char_index_matrix),
        Some(expected_offset)
    );

    assert_eq!(
        file.find_bytes(bytes, &mut preprocess_cache_continuous_hashmap),
        Some(expected_offset)
    );

    assert_eq!(
        file.find_bytes(bytes, &mut preprocess_cache_default),
        Some(expected_offset)
    );
}

#[test]
fn test_rfind_bytes() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file = FileReader::open(tempdir.path().join("test_find_bytes"));
    let bytes = b"test";
    let expected_offset = 23;

    let mut preprocess_cache_windows_hashmap = file.preprocess_with::<WindowsHashmap>();
    let mut preprocess_cache_char_index_matrix = file.preprocess_with::<CharIndexMatrix>();
    let mut preprocess_cache_continuous_hashmap = file.preprocess_with::<ContinuousHashmap>();

    let mut preprocess_cache_default = file.preprocess();

    assert_eq!(
        file.rfind_bytes(bytes, &mut preprocess_cache_windows_hashmap),
        Some(expected_offset)
    );

    assert_eq!(
        file.rfind_bytes(bytes, &mut preprocess_cache_char_index_matrix),
        Some(expected_offset)
    );

    assert_eq!(
        file.rfind_bytes(bytes, &mut preprocess_cache_continuous_hashmap),
        Some(expected_offset)
    );

    assert_eq!(
        file.rfind_bytes(bytes, &mut preprocess_cache_default),
        Some(expected_offset)
    );
}

#[test]
fn test_find_bytes_all() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file = FileReader::open(tempdir.path().join("test_find_bytes"));
    let bytes = b"test";
    let expected_offsets = vec![8, 13, 18, 23];

    let mut preprocess_cache_windows_hashmap = file.preprocess_with::<WindowsHashmap>();
    let mut preprocess_cache_char_index_matrix = file.preprocess_with::<CharIndexMatrix>();
    let mut preprocess_cache_continuous_hashmap = file.preprocess_with::<ContinuousHashmap>();
    let mut preprocess_cache_default = file.preprocess();

    assert_eq!(
        file.find_bytes_all(bytes, &mut preprocess_cache_windows_hashmap),
        Some(expected_offsets.clone())
    );

    assert_eq!(
        file.find_bytes_all(bytes, &mut preprocess_cache_char_index_matrix),
        Some(expected_offsets.clone())
    );

    assert_eq!(
        file.find_bytes_all(bytes, &mut preprocess_cache_continuous_hashmap),
        Some(expected_offsets.clone())
    );

    assert_eq!(
        file.find_bytes_all(bytes, &mut preprocess_cache_default),
        Some(expected_offsets.clone())
    );
}

#[test]
fn test_find_bytes_nth() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file = FileReader::open(tempdir.path().join("test_find_bytes"));
    let bytes = b"test";
    let expected_offset = 13;

    // assert_eq!(file.find_bytes_nth(bytes, 1), Some(expected_offset));

    let mut preprocess_cache_windows_hashmap = file.preprocess_with::<WindowsHashmap>();
    let mut preprocess_cache_char_index_matrix = file.preprocess_with::<CharIndexMatrix>();
    let mut preprocess_cache_continuous_hashmap = file.preprocess_with::<ContinuousHashmap>();
    let mut preprocess_cache_default = file.preprocess();

    assert_eq!(
        file.find_bytes_nth(bytes, 1, &mut preprocess_cache_windows_hashmap),
        Some(expected_offset)
    );

    assert_eq!(
        file.find_bytes_nth(bytes, 1, &mut preprocess_cache_char_index_matrix),
        Some(expected_offset)
    );

    assert_eq!(
        file.find_bytes_nth(bytes, 1, &mut preprocess_cache_continuous_hashmap),
        Some(expected_offset)
    );

    assert_eq!(
        file.find_bytes_nth(bytes, 1, &mut preprocess_cache_default),
        Some(expected_offset)
    );
}

#[test]
fn test_compare_files() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    create_test_files(&tempdir_path);
    let test_file_path = tempdir_path.join("test_file");
    let test_file2_path = tempdir_path.join("test_file2");
    let test_file_diff_path = tempdir_path.join("test_file_diff");
    assert!(FileReader::compare_files(&test_file_path, &test_file2_path));
    assert!(!FileReader::compare_files(
        &test_file_path,
        &test_file_diff_path
    ));
    assert!(!FileReader::compare_files(
        &test_file2_path,
        &test_file_diff_path
    ));
}

#[test]
fn test_compare_to() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    assert!(file_reader.compare_to(tempdir.path().join("test_file2")));
    assert!(!file_reader.compare_to(tempdir.path().join("test_file_diff")));
}

#[test]
fn test_compare_to_file() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    let file = OpenOptions::new()
        .read(true)
        .open(tempdir.path().join("test_file2"))
        .unwrap();
    assert!(file_reader.compare_to_file(&file));
}

#[test]
fn test_compare_hash() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());

    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    let self_hash = file_reader.hash();

    let file_reader_same = FileReader::open(tempdir.path().join("test_file2"));
    let same_hash = file_reader_same.hash();

    let file_reader_diff = FileReader::open(tempdir.path().join("test_file_diff"));
    let diff_hash = file_reader_diff.hash();

    assert!(file_reader.compare_hash::<Sha3_256>(&self_hash));
    assert!(file_reader.compare_hash::<Sha3_256>(&same_hash));
    assert!(!file_reader.compare_hash::<Sha3_256>(&diff_hash));
}

#[test]
fn test_into_iter() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
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
}

#[test]
fn test_into_par_iter() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
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
}

#[test]
fn test_eq() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file_reader = FileReader::open(tempdir.path().join("test_file"));
    let file_reader_same = FileReader::open(tempdir.path().join("test_file2"));
    let file_reader_diff = FileReader::open(tempdir.path().join("test_file_diff"));
    assert_eq!(file_reader, file_reader_same);
    assert_ne!(file_reader, file_reader_diff);
}
