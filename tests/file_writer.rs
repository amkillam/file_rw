use file_rw::{FileReader, FileWriter};
use std::fs::OpenOptions;
use tempfile::tempdir;

macro_rules! file_writer_test {
    ($file_name:expr, $init_text:expr, |$tempdir:ident, $tempdir_path:ident, $test_file_path:ident, $file_writer:ident, $file_reader:ident| $block:block) => {{
        let $tempdir = tempdir().unwrap();
        let $tempdir_path = $tempdir.path();
        let $test_file_path = $tempdir_path.join($file_name);
        let mut $file_writer = FileWriter::open(&$test_file_path);
        $file_writer.overwrite($init_text);
        let $file_reader = FileReader::open(&$test_file_path);
        $block
    }};
}

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
    file_writer_test!(
        "test_overwrite",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            assert_eq!(file_reader.read_to_string(), "Hello, world!");
        }
    );
}

#[test]
fn extend_len_by() {
    file_writer_test!(
        "test_extend_len_by",
        "",
        |tempdir, tempdir_path, test_file_path, file_writer, _file_reader| {
            file_writer.extend_len_by(100);
            assert!(file_writer.len() == 100);
        }
    );
}

#[test]
fn test_set_len() {
    file_writer_test!(
        "test_set_len",
        "",
        |tempdir, tempdir_path, test_file_path, file_writer, _file_reader| {
            file_writer.set_len(100);
            assert!(file_writer.len() == 100);
        }
    );
}

#[test]
fn test_len() {
    file_writer_test!(
        "test_len",
        "",
        |tempdir, tempdir_path, test_file_path, file_writer, _file_reader| {
            file_writer.set_len(100);
            assert!(file_writer.len() == 100);
        }
    );
}

#[test]
fn test_append() {
    file_writer_test!(
        "test_append",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, _file_reader| {
            file_writer.append("Hello, world!");
            assert!(file_writer.len() == 26);
        }
    );
}

#[test]
fn test_open() {
    file_writer_test!(
        "test_open",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            assert_eq!(file_reader.read_to_string(), "Hello, world!");
        }
    );
}

#[test]
fn test_write() {
    file_writer_test!(
        "test_write",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.write("Hello. world.");
            assert_eq!(file_reader.read_to_string(), "Hello. world.");
        }
    );
}

#[test]
fn test_replace() {
    file_writer_test!(
        "test_replace",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.replace("Heyya", 0);
            assert_eq!(file_reader.read_to_string(), "Heyya, world!");
        }
    );
}

#[test]
fn test_find_replace() {
    file_writer_test!(
        "test_find_replace",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.find_replace("Hello", "world");
            assert_eq!(file_reader.read_to_string(), "world, world!");
        }
    );
}

#[test]
fn test_rfind_replace() {
    file_writer_test!(
        "test_rfind_replace",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.rfind_replace("o", "a");
            assert_eq!(file_reader.read_to_string(), "Hello, warld!");
        }
    );
}

#[test]
fn test_find_replace_nth() {
    file_writer_test!(
        "test_find_replace_nth",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.find_replace_nth("o", "a", 1);
            assert_eq!(file_reader.read_to_string(), "Hello, warld!");
        }
    );
}

#[test]
fn test_rfind_replace_nth() {
    file_writer_test!(
        "test_rfind_replace_nth",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.rfind_replace_nth("o", "a", 1);
            assert_eq!(file_reader.read_to_string(), "Hella, world!");
        }
    );
}

#[test]
fn test_find_replace_all() {
    file_writer_test!(
        "test_find_replace_all",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            file_writer.find_replace_all("o", "a");
            assert_eq!(file_reader.read_to_string(), "Hella, warld!");
        }
    );
}

#[test]
fn test_file() {
    file_writer_test!(
        "test_file",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, _file_reader| {
            let file = file_writer.file();
            let file_reader = FileReader::open_file(&file);
            assert_eq!(file_reader.read_to_string(), "Hello, world!");
        }
    );
}

#[test]
fn test_path() {
    file_writer_test!(
        "test_path",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, _file_reader| {
            let path = file_writer.path();
            let file_reader = FileReader::open(path.as_ref());
            assert_eq!(file_reader.read_to_string(), "Hello, world!");
        }
    );
}

#[test]
fn test_mmap() {
    file_writer_test!(
        "test_mmap",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, file_reader| {
            let mmap = file_writer.mmap();
            mmap[..].copy_from_slice(&"Hullo, world!".as_bytes());
            assert_eq!(file_reader.read_to_string(), "Hullo, world!");
        }
    );
}

#[test]
fn test_to_reader() {
    file_writer_test!(
        "test_to_reader",
        "Hello, world!",
        |tempdir, tempdir_path, test_file_path, file_writer, _file_reader| {
            let file_reader = file_writer.to_reader();
            assert_eq!(file_reader.read_to_string(), "Hello, world!");
        }
    );
}
