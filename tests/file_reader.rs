use file_rw::FileReader;
use sha3::{Digest, Sha3_256};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use tempfile::tempdir;

macro_rules! file_reader_test {
    ($file_name:expr, |$tempdir:ident, $tempdir_path:ident, $test_file_path:ident, $file_writer:ident, $file_reader:ident| $block:block) => {{
        let $tempdir = tempdir().unwrap();
        let $tempdir_path = $tempdir.path();
        create_test_files(&$tempdir_path);
        let $test_file_path = $tempdir_path.join($file_name);
        let $file_reader = FileReader::open(&$test_file_path).unwrap();
        let mut $file_writer = $file_reader.to_writer().unwrap();
        $block
    }};
}

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
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            assert_eq!(file_reader.read_to_string(), "test file\n");
        }
    );
}

#[test]
fn test_open_file() {
    let tempdir = tempdir().unwrap();
    create_test_files(tempdir.path());
    let file = OpenOptions::new()
        .read(true)
        .open(tempdir.path().join("test_file"))
        .unwrap();
    let file_reader = FileReader::open_file(&file).unwrap();
    assert_eq!(file_reader.read_to_string(), "test file\n");
}

#[test]
fn test_read_to_string() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            assert_eq!(file_reader.read_to_string(), "test file\n");
        }
    );
}

#[test]
fn test_bytes() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            assert_eq!(file_reader.bytes(), b"test file\n");
        }
    );
}

#[test]
fn test_to_vec() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            assert_eq!(file_reader.to_vec(), b"test file\n");
        }
    );
}

#[test]
fn test_file() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let mut file_text_buffer = String::new();
            file_reader
                .file()
                .unwrap()
                .read_to_string(&mut file_text_buffer)
                .unwrap_or_else(|_| panic!("Could not read file"));
            let expected_text = "test file\n".to_string();
            assert_eq!(file_text_buffer, expected_text);
        }
    );
}

#[test]
fn test_mmap() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            assert_eq!(file_reader.mmap().len(), 10);
        }
    );
}

#[test]
fn test_path() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            assert_eq!(file_reader.path(), &test_file_path);
        }
    );
}

#[test]
fn test_to_writer() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.write(b"testwrite\n");
            assert_eq!(file_reader.read_to_string(), "testwrite\n");
        }
    );
}

#[test]
fn test_hash() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let expected_hash = sha3::Sha3_256::digest(b"test file\n");
            assert_eq!(file_reader.hash(), expected_hash);
        }
    );
}

#[test]
fn test_hash_with() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let expected_hash = sha3::Sha3_256::digest(b"test file\n");
            assert_eq!(file_reader.hash_with::<sha3::Sha3_256>(), expected_hash);
        }
    );
}

#[test]
fn test_hash_to_string() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let expected_string =
                "301285a36e29434a5a13a2c307284e0d64edf827b723290ff4351a6414aa18cf".to_string();
            assert_eq!(file_reader.hash_to_string(), expected_string);
        }
    );
}

#[test]
fn test_find_bytes() {
    file_reader_test!(
        "test_find_bytes",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let bytes = b"test";
            let expected_offset = 8;
            assert_eq!(file_reader.find_bytes(bytes), Some(expected_offset));
        }
    );
}

#[test]
fn test_rfind_bytes() {
    file_reader_test!(
        "test_find_bytes",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let bytes = b"test";
            let expected_offset = 23;

            assert_eq!(file_reader.rfind_bytes(bytes), Some(expected_offset));
        }
    );
}

#[test]
fn test_find_bytes_all() {
    file_reader_test!(
        "test_find_bytes",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let bytes = b"test";
            let expected_offsets = vec![8, 13, 18, 23];

            assert_eq!(file_reader.find_bytes_all(bytes), expected_offsets);
        }
    );
}

#[test]
fn test_rfind_bytes_all() {
    file_reader_test!(
        "test_find_bytes",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let bytes = b"test";
            let expected_offsets = vec![23, 18, 13, 8];

            assert_eq!(file_reader.rfind_bytes_all(bytes), expected_offsets);
        }
    );
}

#[test]
fn test_find_bytes_nth() {
    file_reader_test!(
        "test_find_bytes",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let bytes = b"test";
            let expected_offset = 13;

            assert_eq!(file_reader.find_bytes_nth(bytes, 1), Some(expected_offset));
        }
    );
}

#[test]
fn test_rfind_bytes_nth() {
    file_reader_test!(
        "test_find_bytes",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let bytes = b"test";
            let expected_offset = 18;

            assert_eq!(file_reader.rfind_bytes_nth(bytes, 1), Some(expected_offset));
        }
    );
}

#[test]
fn test_compare_files() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let file_reader_same_path = tempdir_path.join("test_file2");
            let file_reader_diff_path = tempdir_path.join("test_file_diff");
            assert!(FileReader::compare_files(
                &test_file_path,
                &file_reader_same_path
            ));
            assert!(!FileReader::compare_files(
                &test_file_path,
                &file_reader_diff_path
            ));
            assert!(!FileReader::compare_files(
                &file_reader_same_path,
                &file_reader_diff_path
            ));
        }
    );
}

#[test]
fn test_compare_to() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            assert!(file_reader.compare_to(tempdir_path.join("test_file2")));
            assert!(!file_reader.compare_to(tempdir_path.join("test_file_diff")));
        }
    );
}

#[test]
fn test_compare_to_file() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let file = OpenOptions::new()
                .read(true)
                .open(tempdir_path.join("test_file2"))
                .unwrap();
            assert!(file_reader.compare_to_file(&file));
        }
    );
}

#[test]
fn test_compare_hash() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let self_hash = file_reader.hash();
            let file_reader_same = FileReader::open(tempdir_path.join("test_file2")).unwrap();
            let same_hash = file_reader_same.hash();
            let file_reader_diff = FileReader::open(tempdir_path.join("test_file_diff")).unwrap();
            let diff_hash = file_reader_diff.hash();
            assert!(file_reader.compare_hash::<Sha3_256>(&self_hash));
            assert!(file_reader.compare_hash::<Sha3_256>(&same_hash));
            assert!(!file_reader.compare_hash::<Sha3_256>(&diff_hash));
        }
    );
}

#[test]
fn test_into_iter() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
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
    );
}

#[test]
fn test_eq() {
    file_reader_test!(
        "test_file",
        |tempdir, tempdir_path, test_file_path, _file_writer, file_reader| {
            let file_reader_same = FileReader::open(tempdir_path.join("test_file2")).unwrap();
            let file_reader_diff = FileReader::open(tempdir_path.join("test_file_diff")).unwrap();
            assert_eq!(file_reader, file_reader_same);
            assert_ne!(file_reader, file_reader_diff);
        }
    );
}
