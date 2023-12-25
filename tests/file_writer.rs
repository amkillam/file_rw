use file_rw::{
    file,
    preprocess::{CharIndexMatrix, ContinuousHashmap, WindowsHashmap},
    FileReader, FileWriter,
};
use std::fs::OpenOptions;
use tempfile::tempdir;
#[cfg(test)]
#[test]

fn test_open_file() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(tempdir_path.join("test_open_file"))
        .unwrap();
    let mut file_writer = FileWriter::open_file(file);
    file_writer.overwrite("Hello, world!");
    let file_reader = FileReader::open(tempdir_path.join("test_open_file"));
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
}

#[test]
fn test_overwrite() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_overwrite"));
    file_writer.overwrite("Hello, world!");
    let file_reader = FileReader::open(tempdir_path.join("test_overwrite"));
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
}

#[test]
fn extend_len_by() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("extend_len_by"));
    file_writer.extend_len_by(100);
    assert!(file_writer.len() == 100);
}

#[test]
fn test_set_len() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_set_len"));
    file_writer.set_len(100);
    assert!(file_writer.len() == 100);
}

#[test]
fn test_len() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_len"));
    file_writer.set_len(100);
    assert!(file_writer.len() == 100);
}

#[test]
fn test_append() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("append"));
    file_writer.overwrite("Hello, world!");
    file_writer.append("Hello, world!");
    assert!(file_writer.len() == 26);
}

#[test]
fn test_open() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_open"));
    file_writer.overwrite("Hello, world!");
    let file_reader = FileReader::open(tempdir_path.join("test_open"));
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
}

#[test]
fn test_write() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_write"));
    file_writer.append("Hello, world!");
    file_writer.write("Hello. world.");
    let file_reader = FileReader::open(tempdir_path.join("test_write"));
    assert_eq!(file_reader.read_to_string(), "Hello. world.");
}

#[test]
fn test_replace() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_replace"));
    file_writer.overwrite("Hello, world!");
    file_writer.replace("Hello", 0);
    let file_reader = FileReader::open(tempdir_path.join("test_replace"));
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
}

#[test]
fn test_find_replace() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_find_replace"));
    file_writer.overwrite("Hello, world!");
    let file_reader = FileReader::open(tempdir_path.join("test_find_replace"));

    let mut preprocess_cache_windows_hashmap = file_writer.preprocess_with::<WindowsHashmap>();
    let mut preprocess_cache_char_index_matrix = file_writer.preprocess_with::<CharIndexMatrix>();
    let mut preprocess_cache_continuous_hashmap =
        file_writer.preprocess_with::<ContinuousHashmap>();
    let mut preprocess_cache_default = file_writer.preprocess();

    file_writer.find_replace("Hello", "world", &mut preprocess_cache_windows_hashmap);
    assert_eq!(file_reader.read_to_string(), "world, world!");
    file_writer.overwrite("Hello, world!");
    assert_eq!(file_reader.read_to_string(), "Hello, world!");

    file_writer.find_replace("Hello", "world", &mut preprocess_cache_char_index_matrix);
    assert_eq!(file_reader.read_to_string(), "world, world!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace("Hello", "world", &mut preprocess_cache_continuous_hashmap);
    assert_eq!(file_reader.read_to_string(), "world, world!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace("Hello", "world", &mut preprocess_cache_default);
    assert_eq!(file_reader.read_to_string(), "world, world!");
}

#[test]
fn test_find_replace_nth() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_find_replace_nth"));
    file_writer.overwrite("Hello, world!");
    let file_reader = FileReader::open(tempdir_path.join("test_find_replace_nth"));

    let mut preprocess_cache_windows_hashmap = file_writer.preprocess_with::<WindowsHashmap>();
    let mut preprocess_cache_char_index_matrix = file_writer.preprocess_with::<CharIndexMatrix>();
    let mut preprocess_cache_continuous_hashmap =
        file_writer.preprocess_with::<ContinuousHashmap>();
    let mut preprocess_cache_default = file_writer.preprocess();

    assert_eq!(file_reader.read_to_string(), "Hello, world!");
    file_writer.find_replace_nth("o", "a", 1, &mut preprocess_cache_windows_hashmap);
    let file_reader = FileReader::open(tempdir_path.join("test_find_replace_nth"));
    assert_eq!(file_reader.read_to_string(), "Hello, warld!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace_nth("o", "a", 1, &mut preprocess_cache_char_index_matrix);
    assert_eq!(file_reader.read_to_string(), "Hello, warld!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace_nth("l", "y", 0, &mut preprocess_cache_continuous_hashmap);
    assert_eq!(file_reader.read_to_string(), "Heylo, world!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace_nth("o", "a", 1, &mut preprocess_cache_default);
    assert_eq!(file_reader.read_to_string(), "Hello, warld!");
}

#[test]
fn test_find_replace_all() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_find_replace_all"));
    file_writer.overwrite("Hello, world!");

    let mut preprocess_cache_windows_hashmap = file_writer.preprocess_with::<WindowsHashmap>();
    let mut preprocess_cache_char_index_matrix = file_writer.preprocess_with::<CharIndexMatrix>();
    let mut preprocess_cache_continuous_hashmap =
        file_writer.preprocess_with::<ContinuousHashmap>();
    let mut preprocess_cache_default = file_writer.preprocess();

    file_writer.find_replace_all("o", "a", &mut preprocess_cache_windows_hashmap);
    let file_reader = FileReader::open(tempdir_path.join("test_find_replace_all"));
    assert_eq!(file_reader.read_to_string(), "Hella, warld!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace_all("o", "a", &mut preprocess_cache_char_index_matrix);
    assert_eq!(file_reader.read_to_string(), "Hella, warld!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace_all("o", "a", &mut preprocess_cache_continuous_hashmap);
    assert_eq!(file_reader.read_to_string(), "Hella, warld!");
    file_writer.overwrite("Hello, world!");

    file_writer.find_replace_all("o", "a", &mut preprocess_cache_default);
    assert_eq!(file_reader.read_to_string(), "Hella, warld!");
}

#[test]
fn test_file() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_file"));
    file_writer.overwrite("Hello, world!");
    let file = file_writer.file();
    let file_reader = FileReader::open_file(&file);
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
}

#[test]
fn test_path() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_path"));
    file_writer.overwrite(&"Hello, world!");
    let path = file_writer.path();
    let file_reader = FileReader::open(path.as_ref());
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
}

#[test]
fn test_mmap() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_mmap"));
    file_writer.overwrite(&"Hello, world!");
    let mmap = file_writer.mmap();
    mmap[..].copy_from_slice(&"Hullo, world!".as_bytes());
    let file_reader = FileReader::open(tempdir_path.join("test_mmap"));
    assert_eq!(file_reader.read_to_string(), "Hullo, world!");
}

#[test]
fn test_to_reader() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path();
    let mut file_writer = FileWriter::open(tempdir_path.join("test_to_reader"));
    file_writer.overwrite(&"Hello, world!");
    let file_reader = file_writer.to_reader();
    assert_eq!(file_reader.read_to_string(), "Hello, world!");
}
