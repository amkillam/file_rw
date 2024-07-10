use crate::{file::open_as_read, FileWriter};
use memmap2::{Mmap, MmapMut};
use std::{fmt, fs::File, io, path::Path};

#[cfg(feature = "search")]
use memchr::memmem::{find, find_iter, rfind, rfind_iter};
#[cfg(feature = "search")]
/// Finds the first occurrence of a byte sequence in the given data.
/// It takes a byte sequence `pattern` and returns the index of the first occurrence.
/// If the byte sequence is not found, it returns None.
pub(crate) fn find_bytes<B: AsRef<[u8]>, P: AsRef<[u8]>>(bytes: B, pattern: P) -> Option<usize> {
    find(bytes.as_ref(), pattern.as_ref())
}

#[cfg(feature = "search")]
/// Finds the last occurrence of a byte sequence in the given data.
/// It takes a byte sequence `pattern` and returns the index of the last occurrence.
/// If the byte sequence is not found, it returns None.
pub(crate) fn rfind_bytes<B: AsRef<[u8]>, P: AsRef<[u8]>>(bytes: B, pattern: P) -> Option<usize> {
    rfind(bytes.as_ref(), pattern.as_ref())
}

#[cfg(feature = "search")]
/// Finds the nth occurrence of a byte sequence in the given data.
/// It takes a byte sequence `pattern` and an index `n`, and returns the index of the nth occurrence.
/// If the byte sequence is not found, it returns None.
pub(crate) fn find_bytes_nth<B: AsRef<[u8]>, P: AsRef<[u8]>>(
    bytes: B,
    pattern: P,
    n: usize,
) -> Option<usize> {
    find_iter(bytes.as_ref(), pattern.as_ref()).nth(n)
}

#[cfg(feature = "search")]
/// Finds the nth occurrence of a byte sequence in the given data, in reverse order.
pub(crate) fn rfind_bytes_nth<B: AsRef<[u8]>, P: AsRef<[u8]>>(
    bytes: B,
    pattern: P,
    n: usize,
) -> Option<usize> {
    rfind_iter(bytes.as_ref(), pattern.as_ref()).nth(n)
}

#[cfg(feature = "search")]
/// Finds all occurrences of a byte sequence in the file data.
/// It takes a byte sequence `pattern` and returns a vector of indices where the byte sequence is found.
pub(crate) fn find_bytes_all<B: AsRef<[u8]>, P: AsRef<[u8]>>(bytes: B, pattern: P) -> Vec<usize> {
    find_iter(bytes.as_ref(), pattern.as_ref()).collect::<Vec<usize>>()
}

#[cfg(feature = "search")]
/// Finds all occurrences of a byte sequence in the given data, in reverse order.
/// It takes a byte sequence `pattern` and returns a vector of indices where the byte sequence is found.
/// The indices are sorted in reverse order.
pub(crate) fn rfind_bytes_all<B: AsRef<[u8]>, P: AsRef<[u8]>>(bytes: B, pattern: P) -> Vec<usize> {
    rfind_iter(bytes.as_ref(), pattern.as_ref()).collect::<Vec<usize>>()
}

#[cfg(feature = "hashing")]
use digest::{Digest, Output};

#[cfg(feature = "hashing")]
pub fn compare_files_with<H: Digest>(
    file_path1: impl AsRef<Path> + Send + Sync,
    file_path2: impl AsRef<Path> + Send + Sync,
) -> bool {
    if let Ok(file1_reader) = FileReader::open(&file_path1) {
        if let Ok(file2_reader) = FileReader::open(&file_path2) {
            return file1_reader.hash_with::<H>() == file2_reader.hash_with::<H>();
        }
    }
    false
}

#[cfg(feature = "sha3_256")]
use sha3::Sha3_256;
#[cfg(feature = "sha3_256")]
/// Compares two files by their SHA3-256 hashes.
/// It takes two file paths `file_path1` and `file_path2`, and returns true if the files are identical (based on their hashes), false otherwise.
pub fn compare_files(
    file_path1: impl AsRef<Path> + Send + Sync,
    file_path2: impl AsRef<Path> + Send + Sync,
) -> bool {
    if let Ok(file1_reader) = FileReader::open(&file_path1) {
        if let Ok(file2_reader) = FileReader::open(&file_path2) {
            return file1_reader.hash() == file2_reader.hash();
        }
    }
    false
}

#[cfg(feature = "filepath")]
use filepath::FilePath;
#[cfg(feature = "filepath")]
impl FileReader<std::path::PathBuf> {
    /// Opens a file and returns a FileReader for it.
    pub fn open_file(file: &File) -> io::Result<Self> {
        let file_path = file.path()?;
        FileReader::<std::path::PathBuf>::new(file, file_path)
    }
}

/// The FileReader struct represents a file reader that provides high-performance file reading capabilities.
/// It uses memory mapping for efficient access to file data.
pub struct FileReader<P: AsRef<Path> + Send + Sync> {
    pub mmap: Mmap,
    pub path: P,
}

impl<P: AsRef<Path> + Send + Sync> fmt::Display for FileReader<P> {
    /// Displays the path of the file.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.as_ref().display())
    }
}

impl<P: AsRef<Path> + Send + Sync> fmt::Debug for FileReader<P> {
    /// Displays the path of the file.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.as_ref().display())
    }
}

impl<P: AsRef<Path> + Send + Sync> FileReader<P> {
    /// Creates a new FileReader for a given file and path.
    /// It memory maps the file for efficient access.
    fn new(file: &File, path: P) -> io::Result<Self> {
        let mmap = unsafe { Mmap::map(file)? };

        Ok(Self { mmap, path })
    }

    /// Opens a file and returns a FileReader for it.
    /// The file is identified by its path.
    pub fn open(path: P) -> io::Result<Self> {
        let file = open_as_read(path.as_ref())?;
        Self::new(&file, path)
    }

    /// Opens a file at the provided path and returns a FileReader for it.
    pub fn open_file_at_path(file: &File, path: P) -> io::Result<Self> {
        Self::new(file, path)
    }

    /// Reads the entire file to a string.
    pub fn read_to_string(&self) -> String {
        self.bytes().iter().map(|c| *c as char).collect::<String>()
    }

    /// Returns a reference to the FileReader's mmap
    /// This is useful for cases where the mmap needs to be accessed directly.
    pub fn mmap(&self) -> &Mmap {
        &self.mmap
    }

    ///Returns a mutable reference to the FileReader's mmap.
    /// This is useful for cases where the mmap needs to be modified directly.
    /// This fails if the file was not opened for writing, which is the default,
    /// unless the file was manually provided using the FileReader::open_file method.
    pub fn mmap_mut(self) -> io::Result<MmapMut> {
        self.mmap.make_mut()
    }

    /// Returns a slice of bytes representing the file data.
    pub fn bytes(&self) -> &[u8] {
        &self.mmap[..]
    }

    /// Returns a vector of bytes representing the file data.
    pub fn to_vec(&self) -> Vec<u8> {
        self.mmap.to_vec()
    }

    /// Opens the file for reading and returns the File object.
    pub fn file(&self) -> io::Result<File> {
        open_as_read(self.path.as_ref())
    }

    /// Returns the path of the file.
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// Opens the file for writing and returns a FileWriter for it.

    pub fn to_writer(self) -> io::Result<FileWriter<P>> {
        FileWriter::open(self.path)
    }

    ///Directly transmutes FileReader into a FileWriter.
    ///This will fail if the file used in the FileReader was not opened for writing.
    ///If the file was opened only for reading, this will return an error.
    ///By default, unless the file was manually provided using the FileReader::open_file method,
    ///the file will be opened for reading only.
    ///In all other cases, use the FileReader::to_writer method.
    pub fn as_writer(self) -> io::Result<FileWriter<P>> {
        Ok(FileWriter {
            mmap: self.mmap.make_mut()?,
            path: self.path,
        })
    }
    #[cfg(feature = "search")]
    /// Finds the first occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `pattern` and returns the index of the first occurrence.
    /// If the byte sequence is not found, it returns None.
    pub fn find_bytes<B: AsRef<[u8]>>(&self, pattern: B) -> Option<usize> {
        find_bytes(self.bytes(), pattern)
    }

    #[cfg(feature = "search")]
    /// Finds the last occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `pattern` and returns the index of the last occurrence.
    /// If the byte sequence is not found, it returns None.
    pub fn rfind_bytes<B: AsRef<[u8]>>(&self, pattern: B) -> Option<usize> {
        rfind_bytes(self.bytes(), pattern)
    }

    #[cfg(feature = "search")]
    /// Finds the nth occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `pattern` and an index `n`, and returns the index of the nth occurrence.
    /// If the byte sequence is not found, it returns None.
    pub fn find_bytes_nth<B: AsRef<[u8]>>(&self, pattern: B, n: usize) -> Option<usize> {
        find_bytes_nth(self.bytes(), pattern, n)
    }

    #[cfg(feature = "search")]
    /// Finds the nth occurrence of a byte sequence in the file data, in reverse order.
    pub fn rfind_bytes_nth<B: AsRef<[u8]>>(&self, pattern: B, n: usize) -> Option<usize> {
        rfind_bytes_nth(self.bytes(), pattern, n)
    }

    #[cfg(feature = "search")]
    /// Finds all occurrences of a byte sequence in the file data.
    /// It takes a byte sequence `pattern` and returns a vector of indices where the byte sequence is found.
    pub fn find_bytes_all<B: AsRef<[u8]>>(&self, pattern: B) -> Vec<usize> {
        find_bytes_all(self.bytes(), pattern)
    }

    #[cfg(feature = "search")]
    /// Finds all occurrences of a byte sequence in the file data, in reverse order.
    /// It takes a byte sequence `pattern` and returns a vector of indices where the byte sequence is found.
    /// The indices are sorted in reverse order.
    pub fn rfind_bytes_all<B: AsRef<[u8]>>(&self, pattern: B) -> Vec<usize> {
        rfind_bytes_all(self.bytes(), pattern)
    }

    #[cfg(feature = "hashing")]
    /// Compares the hash of the FileReader's file to a given hash.
    /// It takes a hash `hash`, and returns true if the hash of the file is identical to the given hash, false otherwise.
    pub fn compare_hash_with<H: Digest>(&self, hash: &Output<H>) -> bool {
        self.hash_with::<H>() == *hash
    }

    #[cfg(feature = "hashing")]
    /// Computes the hash of the file data using a given hash function.
    pub fn hash_with<H: Digest>(&self) -> Output<H> {
        H::digest(self.bytes())
    }

    #[cfg(feature = "hashing")]
    /// Computes the hash of the file data using the given hash function and returns it as a hex string.
    pub fn hash_to_string_with<H: Digest>(&self) -> String {
        let hash = self.hash_with::<H>();
        hash.iter().fold("".to_string(), |mut acc, byte| {
            acc.push_str(&format!("{:02x}", byte));
            acc
        })
    }

    #[cfg(feature = "hashing")]
    /// Compares the FileReader's file to another file by their hashes outputted by the given
    /// hash function.
    /// It takes a file path `file_path`, and returns true if the files are identical (based on their hashes), false otherwise.
    pub fn compare_to_file_at_path_with<H: Digest>(
        &self,
        file_path: impl AsRef<Path> + Send + Sync,
    ) -> bool {
        if let Ok(file_reader) = FileReader::open(&file_path) {
            self.hash_with::<H>() == file_reader.hash_with::<H>()
        } else {
            false
        }
    }

    #[cfg(all(feature = "hashing", feature = "filepath"))]
    /// Compares the FileReader's file to another file by their hashes outputted by the given hash
    /// function.
    /// It takes a File object `file`, and returns true if the files are identical (based on their hashes), false otherwise.
    pub fn compare_to_file_with<H: Digest>(&self, file: &File) -> bool {
        if let Ok(file_reader) = FileReader::open_file(file) {
            self.hash_with::<H>() == file_reader.hash_with::<H>()
        } else {
            false
        }
    }

    #[cfg(feature = "sha3_256")]
    /// Compares the hash of the FileReader's file to a given hash.
    /// It takes a hash `hash`, and returns true if the hash of the file is identical to the given hash, false otherwise.
    pub fn compare_hash(&self, hash: &Output<Sha3_256>) -> bool {
        self.compare_hash_with::<Sha3_256>(hash)
    }

    #[cfg(feature = "sha3_256")]
    /// Computes the SHA3-256 hash of the file data.
    pub fn hash(&self) -> Output<Sha3_256> {
        self.hash_with::<Sha3_256>()
    }

    #[cfg(feature = "sha3_256")]
    /// Computes the SHA3-256 hash of the file data and returns it as a hex string.
    pub fn hash_to_string(&self) -> String {
        self.hash_to_string_with::<Sha3_256>()
    }

    #[cfg(feature = "sha3_256")]
    /// Compares the FileReader's file to another file by their SHA3-256 hashes.
    /// It takes a file path `file_path`, and returns true if the files are identical (based on their hashes), false otherwise.
    pub fn compare_to_file_at_path(&self, file_path: impl AsRef<Path> + Send + Sync) -> bool {
        self.compare_to_file_at_path_with::<Sha3_256>(file_path)
    }

    #[cfg(feature = "sha3_256")]
    /// Compares the FileReader's file to another file by their SHA3-256 hashes.
    /// It takes a File object `file`, and returns true if the files are identical (based on their hashes), false otherwise.
    pub fn compare_to_file(&self, file: &File) -> bool {
        self.compare_to_file_with::<Sha3_256>(file)
    }
}

impl<P: AsRef<Path> + Send + Sync> IntoIterator for FileReader<P> {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Converts the FileReader into an iterator over the bytes of the file data.
    #[allow(clippy::unnecessary_to_owned)] //Not actually unnecessary in this case
    fn into_iter(self) -> Self::IntoIter {
        self.bytes().to_vec().into_iter()
    }
}

#[cfg(feature = "sha3_256")]
impl<P: AsRef<Path> + Send + Sync> PartialEq for FileReader<P> {
    /// Compares two FileReader instances for equality based on their hashes.
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

#[cfg(not(feature = "sha3_256"))]
impl<P: AsRef<Path> + Send + Sync> PartialEq for FileReader<P> {
    /// Compares two FileReader instances for equality based on their file data.
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}
