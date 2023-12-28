use crate::read::preprocess::preprocessor::{Preprocessor, Search};
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator,
        ParallelIterator,
    },
    slice::ParallelSliceMut,
};

/// A preprocessor that uses a matrix of indices to find byte sequences.
/// All possible bytes from 0x00 - 0xFF are mapped to a vector of indices
/// in an array of length 0xFF where they occur in the file.
/// This allows for fast lookup of byte sequences, and reasonable memory usage
/// and preprocessing time.
/// Space complexity is O(n), where n is the number of bytes in the file.
/// Preprocessing Time complexity is O(n),
/// where n is the number of bytes in the file.
/// Search Time complexity is O(m), where m is the number of bytes in the pattern.
pub struct CharIndexMatrix {
    matrix: Box<[Vec<usize>; 0xFF+1]>,
}

impl CharIndexMatrix {
    //Evaluate byte equality in parallel
    fn find_inner_compare(bytes: &[u8], i: &usize, pattern: &[u8]) -> Option<usize> {
        if bytes.len() < *i + pattern.len() {
            return None;
        }
        if bytes[*i..*i + pattern.len()]
            .par_iter()
            .zip(pattern.par_iter())
            .all(|(a, b)| a == b)
        {
            return Some(*i);
        }
        None
    }
}

const NEW_VEC: Vec<usize> = Vec::new();
impl Preprocessor for CharIndexMatrix {
    fn new(bytes: impl AsRef<[u8]>) -> Self {
        let bytes = bytes.as_ref();
        let lookup_map = bytes
            .par_iter()
            .enumerate()
            .fold(
                || Box::new([NEW_VEC; 0xFF+1]),
                |mut lookup_map: Box<[Vec<usize>; 0xFF+1]>, (i, byte)| {
                    lookup_map[*byte as usize].push(i);
                    lookup_map
                },
            )
            .reduce(
                || Box::new([NEW_VEC; 0xFF+1]),
                |mut lookup_map1, lookup_map2| {
                    lookup_map1
                        .par_iter_mut()
                        .zip(lookup_map2.par_iter())
                        .for_each(|(vec1, vec2)| vec1.extend(vec2));
                    lookup_map1
                },
            );

        Self { matrix: lookup_map }
    }
}

impl Search for CharIndexMatrix {
    /// Finds the first occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the first occurrence.
    /// If the byte sequence is not found, it returns None.
    fn find_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize> {
        let bytes = bytes.as_ref();
        let pattern = pattern.as_ref();

        self.find_bytes_all(bytes, pattern).and_then(|mut v| {
            v.par_sort_unstable();
            v.first().and_then(|i| Some(*i))
        })
    }

    /// Finds the last occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the last occurrence.
    /// If the byte sequence is not found, it returns None.
    fn rfind_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize> {
        let bytes = bytes.as_ref();
        let pattern = pattern.as_ref();

        self.rfind_bytes_all(bytes, pattern)
            .and_then(|v| v.get(0).copied())
    }

    /// Finds all occurrences of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns a vector of indices where the byte sequence is found.
    fn find_bytes_all(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
    ) -> Option<Vec<usize>> {
        let bytes = bytes.as_ref();
        let pattern = pattern.as_ref();
        self.matrix[pattern[0] as usize]
            .par_iter()
            .filter_map(|i| Self::find_inner_compare(bytes, i, pattern))
            .collect::<Vec<usize>>()
            .into()
    }

    /// Finds all occurrences of a byte sequence in the file data, in reverse order.
    /// It takes a byte sequence `bytes` and returns a vector of indices where the byte sequence is found.
    /// The indices are sorted in reverse order.
    fn rfind_bytes_all(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
    ) -> Option<Vec<usize>> {
        self.find_bytes_all(bytes, pattern).and_then(|mut v| {
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
        self.find_bytes_all(bytes, pattern).and_then(|mut v| {
            v.par_sort_unstable();
            v.get(n).copied()
        })
    }

    /// Finds the nth occurrence of a byte sequence in the file data, in reverse order.
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
