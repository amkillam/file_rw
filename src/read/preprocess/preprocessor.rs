pub trait Preprocessor {
    fn new(bytes: impl AsRef<[u8]>) -> Self;
}

/// A trait for searching byte sequences in a file.
/// It is implemented for all types that implement Preprocessor.
/// self must be mutable to allow generic function calls across all preprocessor types -
/// some preprocessors, such as the ContinuousHashmap, require mutable access to the preprocessor,
/// but most do not.
pub trait Search: Preprocessor {
    /// Finds the first occurrence of a byte sequence in the file data.
    fn find_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize>;

    ///Finds all occurrences of a byte sequence in the file data.
    fn find_bytes_all(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
    ) -> Option<Vec<usize>>;

    ///Finds the nth occurrence of a byte sequence in the file data.
    fn find_bytes_nth(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
        n: usize,
    ) -> Option<usize>;

    ///Finds the last occurrence of a byte sequence in the file data.
    fn rfind_bytes(&mut self, bytes: impl AsRef<[u8]>, pattern: impl AsRef<[u8]>) -> Option<usize>;

    ///Finds all occurrences of a byte sequence in the file data, in reverse order.
    fn rfind_bytes_all(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
    ) -> Option<Vec<usize>>;

    ///Finds the nth occurrence of a byte sequence in the file data, in reverse order.
    fn rfind_bytes_nth(
        &mut self,
        bytes: impl AsRef<[u8]>,
        pattern: impl AsRef<[u8]>,
        n: usize,
    ) -> Option<usize>;
}
