use crate::read::preprocess::preprocessor::{Preprocessor, Search};
use crate::read::preprocess::windows_hashmap::hash_windows;
use ahash::AHashMap;
use rayon::slice::ParallelSliceMut;

//No initial preprocssing, but given a pattern,
//Preprocesses all windows of length (m) in the data,
//Where m is the length of the pattern.
//Best case (initial) space complexity is O(1)
//Worst case space complexity is O(n^2), where a pattern has been
//searched with every length 1..n, where n is the length of the data.
//Worst case search time complexity is O(n), occuring if a pattern of length m has not yet been searched.
//Upon searching a pattern with length m, All windows of length m, at each index of n,
//must be hashed (O(n)). The pattern is then found (or not) in O(1) time in the hashmap.
//Best case time complexity is O(1), occuring if a pattern of length m has
//already been searched for, and all windows therefore hashed.
pub struct ContinuousHashmap {
    hashmap: Box<AHashMap<Box<[u8]>, Vec<usize>>>,
    lengths_searched: Box<[bool]>,
}

impl ContinuousHashmap {
    fn process(&mut self, bytes: &[u8], pattern_len: usize) {
        self.hashmap.extend(*hash_windows(bytes, pattern_len));
        self.lengths_searched[pattern_len - 1] = true;
    }

    fn check_processed(&self, pattern_len: usize) -> bool {
        self.lengths_searched[pattern_len - 1]
    }

    fn ensure_processed(&mut self, bytes: &[u8], pattern_len: usize) {
        if !self.check_processed(pattern_len) {
            self.process(bytes, pattern_len);
        }
    }
}

//Instant
impl Preprocessor for ContinuousHashmap {
    fn new(bytes: impl AsRef<[u8]>) -> Self {
        let bytes = bytes.as_ref();
        Self {
            hashmap: Box::new(AHashMap::new()),
            lengths_searched: vec![false; bytes.as_ref().len()].into_boxed_slice(),
        }
    }
}

impl Search for ContinuousHashmap {
    /// Finds the first occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the first occurrence.
    /// If the byte sequence is not found, it returns None.
    fn find_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize> {
        let bytes = bytes.as_ref();
        let pattern = pattern.as_ref();

        self.ensure_processed(bytes, pattern.len());

        self.hashmap.get(pattern).and_then(|v| v.first().copied())
    }

    /// Finds the last occurrence of a byte sequence in the file data.
    /// It takes a byte sequence `bytes` and returns the index of the last occurrence.
    /// If the byte sequence is not found, it returns None.
    fn rfind_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize> {
        let bytes = bytes.as_ref();
        let pattern = pattern.as_ref();
        self.ensure_processed(bytes, pattern.len());
        self.hashmap.get(pattern).and_then(|v| v.last().copied())
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
        self.ensure_processed(bytes, pattern.len());
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
        let bytes = bytes.as_ref();
        let pattern = pattern.as_ref();
        self.ensure_processed(bytes, pattern.len());
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
