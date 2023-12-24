use crate::file::open_as_read;
use crate::FileWriter;
use digest::{Digest, Output};
use filepath::FilePath;
use memmap2::Mmap;
use rayon::prelude::*;
use sha3::Sha3_256;
use std::{fmt, fs::File, path::Path};

/// The FileReader struct represents a file reader that provides high-performance file reading capabilities.
/// It uses memory mapping for efficient access to file data.
pub struct FileReader {
    mmap: Box<Mmap>,
    path: Box<dyn AsRef<Path> + Send + Sync>,
    preprocess_cache: Box<Option<[Vec<usize>; 256]>>,
}

impl fmt::Display for FileReader {
    /// Displays the path of the file.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.as_ref().as_ref().display())
    }
}

impl fmt::Debug for FileReader {
    /// Displays the path of the file.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.as_ref().as_ref().display())
    }
}

impl FileReader {
    /// Creates a new FileReader for a given file and path.
    /// It memory maps the file for efficient access.
    fn new(file: &File, path: impl AsRef<Path> + Send + Sync) -> Self {
        let mmap = Box::new(unsafe {
            Mmap::map(file).unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        });
        Self {
            mmap,
            path: Box::new(path.as_ref().to_path_buf()),
            preprocess_cache: None,
        }
    }

    /// Opens a file and returns a FileReader for it.
    /// The file is identified by its File object.
    pub fn open_file(file: &File) -> Self {
        let file_path = file
            .path()
            .unwrap_or_else(|err| panic!("Could not get path of writer file. Error: {}", err));
        Self::new(file, file_path)
    }

    /// Opens a file and returns a FileReader for it.
    /// The file is identified by its path.
    pub fn open(path: impl AsRef<Path> + Send + Sync) -> Self {
        let file = open_as_read(path.as_ref());
        Self::new(&file, path)
    }

    /// Reads the entire file to a string.
    pub fn read_to_string(&self) -> String {
        self.bytes().iter().map(|c| *c as char).collect::<String>()
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
    pub fn file(&self) -> File {
        open_as_read(self.path.as_ref().as_ref())
    }

    /// Returns the memory-mapped file.
    pub fn mmap(&self) -> &Box<Mmap> {
        &self.mmap
    }

    /// Returns the path of the file.
    pub fn path(&self) -> &Path {
        self.path.as_ref().as_ref()
    }

    /// Opens the file for writing and returns a FileWriter for it.
    pub fn to_writer(&self) -> FileWriter {
        FileWriter::open(&self.path.as_ref())
    }

    /// Computes the hash of the file data using a given hash function.
    pub fn hash_with<H: Digest>(&self) -> Output<H> {
        H::digest(&self.bytes())
    }

    /// Computes the SHA3-256 hash of the file data.
    pub fn hash(&self) -> Output<Sha3_256> {
        self.hash_with::<Sha3_256>()
    }

    /// Computes the hash of the file data and returns it as a hex string.
    pub fn hash_to_string(&self) -> String {
        let mut hash = self.hash();
        hash.par_iter_mut()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    //Cache occurences of every possible 0x00 - 0xFF to know where to start in the future
    //This could be implemented using an aho-corasick automaton, which would be slower
    //in preprocessing, but faster in searching. This is optimised with the inversed tradeoff.
    //Preprocessing is performed with the expectation that only a few searches will be performed,
    //but more than one.
    pub fn preprocess(&self) -> &Self {
        const NEW_VEC: Vec<usize> = Vec::new();
        let lookup_map = self
            .bytes()
            .par_iter()
            .enumerate()
            .fold(
                || [NEW_VEC; 256],
                |mut lookup_map: [Vec<usize>; 256], (i, byte)| {
                    lookup_map[*byte as usize].push(i);
                    lookup_map
                },
            )
            .reduce(
                || [NEW_VEC; 256],
                |mut lookup_map1, lookup_map2| {
                    lookup_map1
                        .par_iter_mut()
                        .zip(lookup_map2.par_iter())
                        .for_each(|(vec1, vec2)| {
                            vec1.extend(vec2);
                            vec1.par_sort_unstable();
                        });
                    lookup_map1
                },
            );

        self.preprocess_cache = Box::new(Some(lookup_map));
        self
    }

    //Evaluate byte equality in parallel
    fn find_inner_compare(&self, i: &usize, bytes: &[u8]) -> Option<usize> {
        if self.bytes()[*i..*i + bytes.len()]
            .par_iter()
            .zip(bytes.par_iter())
            .all(|(a, b)| a == b)
        {
            return Some(*i);
        }
        None
    }

    /// Finds the first occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the first occurrence.
    /// If the byte sequence is not found, it returns None.
    pub fn find_bytes(&self, bytes: &impl AsRef<[u8]>) -> Option<usize> {
        if self.preprocess_cache.is_none() {
            self.preprocess();
        }

        let bytes = bytes.as_ref();

        self.preprocess_cache.as_ref().unwrap()[bytes[0] as usize]
            .par_iter()
            .find_map_first(|i| self.find_inner_compare(i, bytes))
    }

    /// Finds the last occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the last occurrence.
    /// If the byte sequence is not found, it returns None.
    pub fn rfind_bytes(&self, bytes: &impl AsRef<[u8]>) -> Option<usize> {
        if self.preprocess_cache.is_none() {
            self.preprocess();
        }

        let bytes = bytes.as_ref();

        self.preprocess_cache.as_ref().unwrap()[bytes[0] as usize]
            .par_iter()
            .rev()
            .find_map_first(|i| self.find_inner_compare(i, bytes))
    }

    /// Finds all occurrences of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns a vector of indices where the byte sequence is found.
    pub fn find_bytes_all(&self, bytes: &impl AsRef<[u8]>) -> Vec<usize> {
        if self.preprocess_cache.is_none() {
            self.preprocess();
        }

        let bytes = bytes.as_ref();

        self.preprocess_cache.as_ref().unwrap()[bytes[0] as usize]
            .par_iter()
            .filter_map(|i| self.find_inner_compare(i, bytes))
            .collect::<Vec<usize>>()
    }

    /// Finds all occurrences of a byte sequence in the file data, in reverse order.
    /// It takes a byte sequence `bytes` and returns a vector of indices where the byte sequence is found.
    /// The indices are sorted in reverse order.
    pub fn rfind_bytes_all(&self, bytes: &impl AsRef<[u8]>) -> Vec<usize> {
        if self.preprocess_cache.is_none() {
            self.preprocess();
        }

        let bytes = bytes.as_ref();

        self.preprocess_cache.as_ref().unwrap()[bytes[0] as usize]
            .par_iter()
            .rev()
            .filter_map(|i| self.find_inner_compare(i, bytes))
            .collect::<Vec<usize>>()
    }

    /// Finds the nth occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and an index `n`, and returns the index of the nth occurrence.
    /// If the byte sequence is not found, it returns None.
    pub fn find_bytes_nth(&self, bytes: &impl AsRef<[u8]>, n: usize) -> Option<usize> {
        //There are two good approaches to this - the nth match could be found by iterating
        //sequentially, then returning upon finding the nth match, or by finding all matches in
        //parallel, then sorting and returning the nth match. The second approach will generally be
        //faster, despite the obvious overhead/inefficiency of finding all matches first, then sorting, because it can be parallelized.
        //
        //This was initially implemented by breaking and returning the nth match in parallel
        //instead, but the nth match found when not parsing data sequentially is not guaranteed to
        //be the nth match in the file, so this was changed to the current approach.

        let mut offsets = self.find_bytes_all(bytes);
        offsets.par_sort_unstable(); //offsets will not have overlapping values
        offsets.get(n).copied()
    }

    /// Compares two files by their hashes.
    /// It takes two file paths `file_path1` and `file_path2`, and returns true if the files are identical (based on their hashes), false otherwise.
    pub fn compare_files(
        file_path1: impl AsRef<Path> + Send + Sync,
        file_path2: impl AsRef<Path> + Send + Sync,
    ) -> bool {
        let file1_reader = FileReader::open(&file_path1);
        let file2_reader = FileReader::open(&file_path2);
        file1_reader.hash() == file2_reader.hash()
    }

    /// Compares the FileReader's file to another file by their hashes.
    /// It takes a file path `file_path`, and returns true if the files are identical (based on their hashes), false otherwise.
    pub fn compare_to(&self, file_path: impl AsRef<Path> + Send + Sync) -> bool {
        let file_reader = FileReader::open(&file_path);
        self.hash() == file_reader.hash()
    }

    /// Compares the FileReader's file to another file by their hashes.
    /// It takes a File object `file`, and returns true if the files are identical (based on their hashes), false otherwise.
    pub fn compare_to_file(&self, file: &File) -> bool {
        let file_reader = FileReader::open_file(&file);
        self.hash() == file_reader.hash()
    }

    /// Compares the hash of the FileReader's file to a given hash.
    /// It takes a hash `hash`, and returns true if the hash of the file is identical to the given hash, false otherwise.
    pub fn compare_hash<T: Digest>(&self, hash: &Output<T>) -> bool {
        self.hash_with::<T>() == *hash
    }
}

impl IntoIterator for FileReader {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Converts the FileReader into an iterator over the bytes of the file data.
    fn into_iter(self) -> Self::IntoIter {
        self.bytes().to_vec().into_iter()
    }
}

impl IntoParallelIterator for FileReader {
    type Item = u8;
    type Iter = rayon::vec::IntoIter<Self::Item>;

    /// Converts the FileReader into a parallel iterator over the bytes of the file data.
    fn into_par_iter(self) -> Self::Iter {
        self.bytes().to_vec().into_par_iter()
    }
}

impl PartialEq for FileReader {
    /// Compares two FileReader instances for equality based on their hashes.
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}
