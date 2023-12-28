use file_rw::file::{open_as_read, open_as_write};
use std::io::Write;
use tempfile::tempdir;

macro_rules! file_test {
    ($file_name:expr, $init_text:expr, |$tempdir:ident, $tempdir_path:ident, $test_file_path:ident, $file:ident| $block:block) => {{
        let $tempdir = tempdir().unwrap();
        let $tempdir_path = $tempdir.path();
        let $test_file_path = $tempdir_path.join($file_name);
        let mut $file = open_as_write(&$test_file_path);
        $file.write_all($init_text.as_bytes()).unwrap();
        assert!($test_file_path.exists());
        $block
    }};
}
#[cfg(test)]
#[test]
fn test_open_as_write() {
    file_test!(
        "test_open_as_write",
        "test",
        |tempdir, tempdir_path, test_file_path, file| {
            assert!(file.metadata().unwrap().len() > 0);
        }
    );
}

#[test]
fn test_open_as_read() {
    file_test!(
        "test_open_as_read",
        "test",
        |tempdir, tempdir_path, test_file_path, file| {
            let file = open_as_read(&test_file_path);
            assert!(file.metadata().unwrap().len() > 0);
        }
    );
}
