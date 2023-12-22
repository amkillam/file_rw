use crate::{
    file::{open_as_append, open_as_write},
    FileReader,
};
use filepath::FilePath;
use memmap2::MmapMut;
use std::{fmt, fs::File, path::Path};

/// `FileWriter` is a structure that allows writing to a file.
/// It uses memory-mapped files for efficient file manipulation.
pub struct FileWriter {
    mmap: Box<MmapMut>,
    path: Box<dyn AsRef<Path> + Send + Sync>,
}

/// Writes "FileWriter({path})" to the provided formatter.
impl fmt::Display for FileWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileWriter({})", self.path.as_ref().as_ref().display())
    }
}

/// Writes "FileWriter({Path})" to the provided formatter.
impl fmt::Debug for FileWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileWriter({})", self.path.as_ref().as_ref().display())
    }
}

impl FileWriter {
    /// Creates a new `FileWriter` instance.
    /// It takes a reference to a `File` and a path, and maps the file into memory.
    fn new<'a>(file: &File, path: impl AsRef<Path> + Send + Sync + fmt::Debug + 'static) -> Self {
        let mmap = Box::new(unsafe {
            MmapMut::map_mut(file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        });

        Self {
            mmap,
            path: Box::new(path),
        }
    }

    /// Opens a file and returns a `FileWriter` instance.
    /// It panics if it cannot get the path of the writer file.
    pub fn open_file(file: File) -> Self {
        let path = file
            .path()
            .unwrap_or_else(|err| panic!("Could not get path of writer file. Error: {}", err));

        Self::new(&file, path)
    }

    /// Opens a file in write mode and returns a `FileWriter` instance.
    pub fn open(path: impl AsRef<Path> + Send + Sync) -> Self {
        let file = open_as_write(path.as_ref());
        FileWriter::open_file(file)
    }

    /// Opens a file in append mode and returns a `FileWriter` instance.
    pub fn open_append(path: impl AsRef<Path> + Send + Sync) -> Self {
        let file = open_as_append(path.as_ref());
        FileWriter::open_file(file)
    }

    /// Writes bytes to the file.
    /// It replaces the entire content of the file with the provided bytes.
    pub fn write(&mut self, bytes: &impl AsRef<[u8]>) -> &Self {
        self.mmap[..].clone_from_slice(bytes.as_ref());
        self
    }

    /// Replaces a portion of the file content starting from the provided offset with the provided bytes.
    pub fn replace(&mut self, bytes: &impl AsRef<[u8]>, offset: usize) -> &Self {
        let bytes = bytes.as_ref();
        self.mmap[offset..offset + bytes.len()].clone_from_slice(bytes);
        self
    }

    /// Finds a sequence of bytes in the file and replaces it with another sequence of bytes.
    /// If the sequence to find is not found, it does nothing.
    pub fn find_replace(&mut self, find: &impl AsRef<[u8]>, replace: &impl AsRef<[u8]>) -> &Self {
        let find = find.as_ref();
        let replace = replace.as_ref();
        let file_reader = FileReader::open(&*(self.path.as_ref().as_ref()));
        let offset = file_reader.find_bytes(&find);

        match offset {
            Some(offset) => {
                self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
            }
            None => (),
        }

        self
    }

    /// Finds the nth occurrence of a slice of bytes in the file and replaces it with another slice of bytes.
    /// If the slice to find is not found, no replacement occurs.
    pub fn find_replace_nth(
        &mut self,
        find: &impl AsRef<[u8]>,
        replace: &impl AsRef<[u8]>,
        n: usize,
    ) -> &Self {
        let replace = replace.as_ref();
        let file_reader = FileReader::open(self.path.as_ref());
        let offset = file_reader.find_bytes_nth(&find, n);
        match offset {
            Some(offset) => {
                self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
            }
            None => (),
        }
        self
    }

    /// Finds all occurrences of a slice of bytes in the file and replaces them with another slice of bytes.
    pub fn find_replace_all(
        &mut self,
        find: &impl AsRef<[u8]>,
        replace: &impl AsRef<[u8]>,
    ) -> &Self {
        let replace = &replace.as_ref();
        let file_reader = FileReader::open(self.path.as_ref());
        let find_results = file_reader.find_bytes_all(find);
        for offset in find_results {
            let _ = &mut self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
        }
        self
    }

    /// Returns a `File` object that represents the file being written to.
    pub fn file(&mut self) -> File {
        open_as_write(self.path.as_ref().as_ref())
    }

    /// Returns a reference to the path of the file being written to.
    pub fn path(&mut self) -> &Box<dyn AsRef<Path> + Send + Sync> {
        &self.path
    }

    /// Returns a mutable reference to the memory-mapped file.
    pub fn mmap(&mut self) -> &mut Box<MmapMut> {
        &mut self.mmap
    }

    /// Converts the `FileWriter` into a `FileReader`.
    pub fn to_reader(&mut self) -> FileReader {
        FileReader::open(self.path.as_ref())
    }
}
