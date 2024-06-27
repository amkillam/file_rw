use crate::{file::open_as_write, FileReader};
use filepath::FilePath;
use memmap2::MmapMut;
use std::{fmt, fs::File, io, path::Path};

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
    fn new(
        file: &File,
        path: impl AsRef<Path> + Send + Sync + fmt::Debug + 'static,
    ) -> io::Result<Self> {
        let mmap = Box::new(unsafe { MmapMut::map_mut(file)? });

        Ok(Self {
            mmap,
            path: Box::new(path),
        })
    }

    /// Opens a file and returns a `FileWriter` instance.
    /// It panics if it cannot get the path of the writer file.
    pub fn open_file(file: File) -> io::Result<Self> {
        let path = file.path()?;

        Self::new(&file, path)
    }

    /// Opens a file in write mode and returns a `FileWriter` instance.
    pub fn open(path: impl AsRef<Path> + Send + Sync) -> io::Result<Self> {
        let file = open_as_write(path.as_ref())?;
        FileWriter::open_file(file)
    }

    /// Writes bytes to the file.
    /// It replaces the entire content of the file with the provided bytes.
    pub fn write(&mut self, bytes: impl AsRef<[u8]>) -> &Self {
        self.mmap[..].clone_from_slice(bytes.as_ref());
        self
    }

    pub fn write_to_offset(&mut self, bytes: impl AsRef<[u8]>, offset: usize) -> &Self {
        self.mmap[offset..offset + bytes.as_ref().len()].clone_from_slice(bytes.as_ref());
        self
    }

    pub fn append(&mut self, bytes: impl AsRef<[u8]>) -> io::Result<&Self> {
        let current_len = self.mmap.len();
        let bytes = bytes.as_ref();
        let new_len = current_len + bytes.len();
        self.set_len(new_len as u64)?;
        self.mmap[current_len..new_len].clone_from_slice(bytes);
        Ok(self)
    }

    pub fn overwrite(&mut self, bytes: impl AsRef<[u8]>) -> io::Result<&Self> {
        let bytes = bytes.as_ref();
        let len = bytes.len();
        self.set_len(len as u64)?;
        self.write(bytes);
        Ok(self)
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.mmap[..]
    }

    pub fn bytes(&self) -> &[u8] {
        &self.mmap[..]
    }

    /// Replaces a portion of the file content starting from the provided offset with the provided bytes.
    pub fn replace(&mut self, bytes: impl AsRef<[u8]>, offset: usize) -> &Self {
        let bytes = bytes.as_ref();
        self.mmap[offset..offset + bytes.len()].clone_from_slice(bytes);
        self
    }

    fn find_replace_inner(
        &mut self,
        find: &[u8],
        replace: &[u8],
        offset: usize,
    ) -> io::Result<&Self> {
        if replace.len() > find.len() {
            let current_bytes = self.mmap[offset + find.len()..].to_vec();
            self.extend_len_by((replace.len() - find.len()) as u64)?;
            self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
            self.mmap[offset + replace.len()..].clone_from_slice(&current_bytes);
        } else {
            self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
        }
        Ok(self)
    }

    /// Finds a sequence of bytes in the file and replaces it with another sequence of bytes.
    /// If the sequence to find is not found, it does nothing.
    pub fn find_replace(
        &mut self,
        find: impl AsRef<[u8]>,
        replace: impl AsRef<[u8]>,
    ) -> io::Result<&Self> {
        let find = find.as_ref();
        let replace = replace.as_ref();
        if let Ok(file_reader) = self.to_reader() {
            if let Some(offset) = file_reader.find_bytes(find) {
                self.find_replace_inner(find, replace, offset)?;
            }
        }

        Ok(self)
    }

    /// Finds the last occurrence of a slice of bytes in the file and replaces it with another slice of bytes.
    pub fn rfind_replace(
        &mut self,
        find: impl AsRef<[u8]>,
        replace: impl AsRef<[u8]>,
    ) -> io::Result<&Self> {
        let find = find.as_ref();
        let replace = replace.as_ref();
        if let Ok(file_reader) = self.to_reader() {
            if let Some(offset) = file_reader.rfind_bytes(find) {
                self.find_replace_inner(find, replace, offset)?;
            }
        }
        Ok(self)
    }

    /// Finds the nth occurrence of a slice of bytes in the file, in reverse order, and replaces it with another slice of bytes.
    pub fn rfind_replace_nth(
        &mut self,
        find: impl AsRef<[u8]>,
        replace: impl AsRef<[u8]>,
        n: usize,
    ) -> io::Result<&Self> {
        let find = find.as_ref();
        let replace = replace.as_ref();
        if let Ok(file_reader) = self.to_reader() {
            if let Some(offset) = file_reader.rfind_bytes_nth(find, n) {
                self.find_replace_inner(find, replace, offset)?;
            }
        }
        Ok(self)
    }

    /// Finds the nth occurrence of a slice of bytes in the file and replaces it with another slice of bytes.
    /// If the slice to find is not found, no replacement occurs.
    pub fn find_replace_nth(
        &mut self,
        find: impl AsRef<[u8]>,
        replace: impl AsRef<[u8]>,
        n: usize,
    ) -> io::Result<&Self> {
        let find = find.as_ref();
        let replace = replace.as_ref();
        if let Ok(file_reader) = self.to_reader() {
            if let Some(offset) = file_reader.find_bytes_nth(find, n) {
                self.find_replace_inner(find, replace, offset)?;
            }
        }
        Ok(self)
    }

    /// Finds all occurrences of a slice of bytes in the file and replaces them with another slice of bytes.
    pub fn find_replace_all(
        &mut self,
        find: impl AsRef<[u8]>,
        replace: impl AsRef<[u8]>,
    ) -> io::Result<&Self> {
        let find = find.as_ref();
        let replace = replace.as_ref();
        if let Ok(file_reader) = self.to_reader() {
            let find_results = file_reader.find_bytes_all(find);
            for offset in &find_results {
                self.find_replace_inner(find, replace, offset.to_owned())?;
            }
        }
        Ok(self)
    }

    /// Returns a `File` object that represents the file being written to.
    pub fn file(&mut self) -> io::Result<File> {
        open_as_write(self.path.as_ref().as_ref())
    }

    pub fn len(&mut self) -> u64 {
        if let Ok(file) = self.file() {
            file.metadata().unwrap().len()
        } else {
            0
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    pub fn set_len(&mut self, len: u64) -> io::Result<&mut Self> {
        let file = self.file()?;
        file.set_len(len)?;
        self.mmap = Box::new(unsafe { MmapMut::map_mut(&file)? });
        Ok(self)
    }

    pub fn extend_len_by(&mut self, len: u64) -> io::Result<&mut Self> {
        let current_len = self.len();
        let new_len = current_len + len;
        self.set_len(new_len)?;
        Ok(self)
    }

    /// Returns a reference to the path of the file being written to.
    pub fn path(&mut self) -> &(dyn AsRef<Path> + Send + Sync) {
        &*self.path
    }

    /// Returns a mutable reference to the memory-mapped file.
    pub fn mmap(&mut self) -> &mut MmapMut {
        &mut self.mmap
    }

    /// Converts the `FileWriter` into a `FileReader`.
    pub fn to_reader(&mut self) -> io::Result<FileReader> {
        FileReader::open(self.path.as_ref())
    }
}
