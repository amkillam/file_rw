use crate::{file::open_as_write, FileReader};
use memmap2::{Mmap, MmapMut};
use std::{fmt, fs::File, io, path::Path};

/// `FileWriter` is a structure that allows writing to a file.
/// It uses memory-mapped files for efficient file manipulation.
pub struct FileWriter<P: AsRef<Path> + Send + Sync> {
    pub mmap: MmapMut,
    pub path: P,
}

/// Writes "FileWriter({path})" to the provided formatter.
impl<P: AsRef<Path> + Send + Sync> fmt::Display for FileWriter<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileWriter({})", self.path.as_ref().display())
    }
}

/// Writes "FileWriter({Path})" to the provided formatter.
impl<P: AsRef<Path> + Send + Sync> fmt::Debug for FileWriter<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileWriter({})", self.path.as_ref().display())
    }
}

#[cfg(feature = "filepath")]
use filepath::FilePath;
#[cfg(feature = "filepath")]
impl FileWriter<std::path::PathBuf> {
    /// Opens a file and returns a `FileWriter` instance.
    pub fn open_file(file: &File) -> io::Result<Self> {
        let path = file.path()?;

        Self::new(file, path)
    }
}

impl<P: AsRef<Path> + Send + Sync> FileWriter<P> {
    /// Creates a new `FileWriter` instance.
    /// It takes a reference to a `File` and a path, and maps the file into memory.
    fn new(file: &File, path: P) -> io::Result<Self> {
        let mmap = unsafe { MmapMut::map_mut(file)? };

        Ok(Self { mmap, path })
    }

    /// Opens a file at the provided path and returns a `FileWriter` instance.
    pub fn open_file_at_path(file: &File, path: P) -> io::Result<Self> {
        Self::new(file, path)
    }

    /// Opens a file in write mode and returns a `FileWriter` instance.
    pub fn open(path: P) -> io::Result<Self> {
        let file = open_as_write(path.as_ref())?;
        Self::new(&file, path)
    }

    /// Writes bytes to the file.
    /// It replaces the entire content of the file with the provided bytes.
    pub fn write<B: AsRef<[u8]>>(&mut self, bytes: B) -> &Self {
        self.mmap[..].copy_from_slice(bytes.as_ref());
        self
    }

    /// Writes bytes to the file at the provided offset.
    pub fn write_to_offset<B: AsRef<[u8]>>(&mut self, bytes: B, offset: usize) -> &Self {
        self.mmap[offset..offset + bytes.as_ref().len()].copy_from_slice(bytes.as_ref());
        self
    }

    /// Appends bytes to the file, extending the file length if necessary.
    pub fn append<B: AsRef<[u8]>>(&mut self, bytes: B) -> io::Result<&Self> {
        let current_len = self.mmap.len();
        let bytes = bytes.as_ref();
        let new_len = current_len + bytes.len();
        self.set_len(new_len as u64)?;
        self.mmap[current_len..new_len].copy_from_slice(bytes);
        Ok(self)
    }

    /// Overwrites the entire content of the file with the provided bytes. The file's length is
    /// extended if the provided bytes are longer than the current file length.
    pub fn overwrite<B: AsRef<[u8]>>(&mut self, bytes: B) -> io::Result<&Self> {
        let bytes = bytes.as_ref();
        let len = bytes.len();
        self.set_len(len as u64)?;
        self.write(bytes);
        Ok(self)
    }

    ///Returns length of the file data
    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    /// Returns a mutable reference to the bytes of the file.
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.mmap[..]
    }

    /// Returns an immutable reference to the bytes of the file.
    pub fn bytes(&self) -> &[u8] {
        &self.mmap[..]
    }

    /// Replaces a portion of the file content starting from the provided offset with the provided bytes.
    pub fn replace<B: AsRef<[u8]>>(&mut self, bytes: B, offset: usize) -> &Self {
        let bytes = bytes.as_ref();
        self.mmap[offset..offset + bytes.len()].copy_from_slice(bytes);
        self
    }

    #[cfg(feature = "search")]
    /// Finds a sequence of bytes in the file and replaces it with another sequence of bytes. If the sequence to find is not found, it does nothing.
    /// If the sequence would be written past the length of the file, the file is extended to accommodate the new bytes.
    fn find_replace_inner<B: AsRef<[u8]>, BO: AsRef<[u8]>>(
        &mut self,
        find: B,
        replace: BO,
        offset: usize,
    ) -> io::Result<&Self> {
        let find = find.as_ref();
        let replace = replace.as_ref();
        if replace.len() > find.len() {
            let current_bytes = self.mmap[offset + find.len()..].to_vec();
            self.extend_len_by((replace.len() - find.len()) as u64)?;
            self.mmap[offset..offset + replace.len()].copy_from_slice(replace);
            self.mmap[offset + replace.len()..].copy_from_slice(&current_bytes);
        } else {
            self.mmap[offset..offset + replace.len()].copy_from_slice(replace);
        }
        Ok(self)
    }

    #[cfg(feature = "search")]
    /// Finds a sequence of bytes in the file and replaces it with another sequence of bytes.
    /// If the sequence to find is not found, it does nothing.
    pub fn find_replace<B: AsRef<[u8]>, BO: AsRef<[u8]>>(
        &mut self,
        find: B,
        replace: BO,
    ) -> io::Result<&Self> {
        if let Some(offset) = crate::read::find_bytes(self.bytes(), &find) {
            self.find_replace_inner(&find, &replace, offset)?;
        }

        Ok(self)
    }

    #[cfg(feature = "search")]
    /// Finds the last occurrence of a slice of bytes in the file and replaces it with another slice of bytes.
    pub fn rfind_replace<B: AsRef<[u8]>, BO: AsRef<[u8]>>(
        &mut self,
        find: B,
        replace: BO,
    ) -> io::Result<&Self> {
        if let Some(offset) = crate::read::rfind_bytes(self.bytes(), &find) {
            self.find_replace_inner(&find, &replace, offset)?;
        }
        Ok(self)
    }

    #[cfg(feature = "search")]
    /// Finds the nth occurrence of a slice of bytes in the file, in reverse order, and replaces it with another slice of bytes.
    pub fn rfind_replace_nth<B: AsRef<[u8]>, BO: AsRef<[u8]>>(
        &mut self,
        find: B,
        replace: BO,
        n: usize,
    ) -> io::Result<&Self> {
        if let Some(offset) = crate::read::rfind_bytes_nth(self.bytes(), &find, n) {
            self.find_replace_inner(&find, &replace, offset)?;
        }
        Ok(self)
    }

    #[cfg(feature = "search")]
    /// Finds the nth occurrence of a slice of bytes in the file and replaces it with another slice of bytes.
    /// If the slice to find is not found, no replacement occurs.
    pub fn find_replace_nth<B: AsRef<[u8]>, BO: AsRef<[u8]>>(
        &mut self,
        find: B,
        replace: BO,
        n: usize,
    ) -> io::Result<&Self> {
        if let Some(offset) = crate::read::find_bytes_nth(self.bytes(), &find, n) {
            self.find_replace_inner(&find, &replace, offset)?;
        }
        Ok(self)
    }

    #[cfg(feature = "search")]
    /// Finds all occurrences of a slice of bytes in the file and replaces them with another slice of bytes.
    pub fn find_replace_all<B: AsRef<[u8]>, BO: AsRef<[u8]>>(
        &mut self,
        find: B,
        replace: BO,
    ) -> io::Result<&Self> {
        for offset in &crate::read::find_bytes_all(self.bytes(), &find) {
            self.find_replace_inner(&find, &replace, offset.to_owned())?;
        }
        Ok(self)
    }

    /// Returns a `File` object that represents the file being written to.
    pub fn file(&mut self) -> io::Result<File> {
        open_as_write(self.path.as_ref())
    }

    /// Checks if the file has a length of zero.
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    /// Sets the length of the file.
    pub fn set_len(&mut self, len: u64) -> io::Result<&mut Self> {
        let file = self.file()?;
        file.set_len(len)?;
        self.mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(self)
    }

    /// Extends the length of the file by the provided length.
    pub fn extend_len_by(&mut self, len: u64) -> io::Result<&mut Self> {
        let current_len = self.len();
        let new_len = current_len + len;
        self.set_len(new_len)?;
        Ok(self)
    }

    /// Returns a reference to the path of the file being written to.
    pub fn path(&mut self) -> &P {
        &self.path
    }

    /// Returns a mutable reference to the memory-mapped file.
    pub fn mmap_mut(&mut self) -> &mut MmapMut {
        &mut self.mmap
    }

    /// Returns an immutable reference to the memory-mapped file.
    /// This fails if the file backing the mmap was not opened as both read and write.
    /// By default, unless the file was manually provided using FileWriter::open_file, the file is opened as both read and write.
    pub fn mmap(self) -> io::Result<Mmap> {
        self.mmap.make_read_only()
    }

    /// Converts the `FileWriter` into a `FileReader` by opening the file as read-only.
    pub fn to_reader(self) -> io::Result<FileReader<P>> {
        FileReader::open(self.path)
    }

    /// Converts the `FileWriter` into a `FileReader`.
    /// This fails if the file backing the mmap was not opened as both read and write.
    /// By default, unless the file was manually provided using FileWriter::open_file, the file is opened as both read and write.
    /// In the event that a file was opened as write-only and passed to FileWriter::open_file, use
    /// FileWriter::to_reader instead
    pub fn as_reader(self) -> io::Result<FileReader<P>> {
        Ok(FileReader {
            mmap: self.mmap.make_read_only()?,
            path: self.path,
        })
    }
}
