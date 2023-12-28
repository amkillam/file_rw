use crate::read::preprocess::preprocessor::{Preprocessor, Search};
use ahash::AHashMap;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};

/// A preprocessor that uses a hashmap of all possible subsequences of
/// given data to search byte sequences.
/// O(n^2) preprocessing time, O(n^2) space complexity. O(1) search time
pub fn hash_windows(bytes: &[u8], pattern_len: usize) -> Box<AHashMap<Box<[u8]>, Vec<usize>>> {
    let bytes_len = bytes.len();
    bytes
        .par_windows(pattern_len)
        .enumerate()
        .fold(
            || Box::new(AHashMap::with_capacity(bytes_len)),
            |mut lookup_map, (j, window)| {
                lookup_map
                    .entry(Box::from(window))
                    .or_insert_with(Vec::new)
                    .push(j);
                lookup_map
            },
        )
        .reduce(
            || Box::new(AHashMap::with_capacity(bytes_len)),
            |mut lookup_map1, lookup_map2| {
                for (key, value) in *lookup_map2 {
                    lookup_map1
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .extend(value);
                }
                lookup_map1
            },
        )
}

/// A preprocessor that uses a hashmap of all possible subsequences of
/// given data to search byte sequences.
/// O(n^2) preprocessing time, O(n^2) space complexity.
/// O(1) search time
pub struct WindowsHashmap {
    hashmap: Box<AHashMap<Box<[u8]>, Vec<usize>>>,
}

impl Preprocessor for WindowsHashmap {
    fn new(bytes: impl AsRef<[u8]>) -> Self {
        let bytes = bytes.as_ref();
        let bytes_len = bytes.len();
        let lookup_map: Box<AHashMap<Box<[u8]>, Vec<usize>>> = (1..bytes_len)
            .into_par_iter()
            .map(|pattern_len| {
                hash_windows(bytes, pattern_len)
            }).reduce(
                || Box::new(AHashMap::with_capacity(bytes_len)),
                |mut lookup_map1, lookup_map2| {
                    for (key, value) in *lookup_map2 {
                        lookup_map1
                            .entry(key)
                            .or_insert_with(Vec::new)
                            .extend(value);
                    }
                    lookup_map1
                },
            );
        Self {
            hashmap: lookup_map,
        }
    }
}

///Implements methods of byte sub-sequence searching
/// bytes arg is not used, but is required for generic function calls
impl Search for WindowsHashmap {
    /// Finds the first occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the first occurrence.
    /// If the byte sequence is not found, it returns None.
    fn find_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize> {
        let _ = bytes;
        let pattern = pattern.as_ref();
        self.hashmap.get(pattern).and_then(|v| v.first().copied())
    }

    /// Finds the last occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the last occurrence.
    /// If the byte sequence is not found, it returns None.
    fn rfind_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize> {
        let _ = bytes;
        let pattern = pattern.as_ref();
        self.hashmap.get(pattern).and_then(|v| v.last().copied())
    }

    /// Finds all occurrences of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns a vector of indices where the byte sequence is found.
    fn find_bytes_all(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
    ) -> Option<Vec<usize>> {
        let _ = bytes;
        let pattern = pattern.as_ref();
        self.hashmap.get(pattern).map(|v| v.to_vec())
    }

    /// Finds all occurrences of a byte sequence in the file data, in reverse order.
    /// It takes a byte sequence `bytes` and returns a vector of indices where the byte sequence is found.
    /// The indices are sorted in reverse order.
    fn rfind_bytes_all(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
    ) -> Option<Vec<usize>> {
        let _ = bytes;
        let pattern = pattern.as_ref();
        self.hashmap.get(pattern).and_then(|v| {
            let mut v = v.to_vec();
            v.par_sort_unstable_by(|a, b| b.cmp(a));
            Some(v)
        })
    }

    /// Finds the nth occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and an index `n`, and returns the index of the nth occurrence.
    /// If the byte sequence is not found, it returns None.
    fn find_bytes_nth(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
        n: usize,
    ) -> Option<usize> {
        self.find_bytes_all(bytes, pattern)
            .and_then(|v| v.get(n).copied())
    }

    fn rfind_bytes_nth(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
        n: usize,
    ) -> Option<usize> {
        self.rfind_bytes_all(bytes, pattern)
            .and_then(|v| v.get(n).copied())
    }
}
