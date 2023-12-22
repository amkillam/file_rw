use rayon::prelude::*;
use crate::{FileWriter, BytesRef};
use memmap2::Mmap;
use digest::{Output, Digest};
use sha3::Sha3_256;
use crate::file::open_as_read;
use std::fs::File;
use std::path::Path;
use filepath::FilePath;

pub struct FileReader {
    mmap: Box<Mmap>,
    path: Box<dyn AsRef<Path> + Send + Sync>
}

impl FileReader {

    fn new(file: &File, path: impl AsRef<Path> + Send + Sync) -> Self {
        let mmap = Box::new(unsafe {
            Mmap::map(file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        });
        Self {
            mmap,
           path:  Box::new(path.as_ref().to_path_buf())
        }
    }

    pub fn open_file(file: &File) -> Self {
        let file_path = file.path().unwrap_or_else(
            |err| panic!("Could not get path of writer file. Error: {}", err));
        Self::new(file, file_path)
    }

    pub fn open(path: impl AsRef<Path> + Send + Sync) -> Self {
        let file = open_as_read(path.as_ref());
        Self::new(&file, path)
    }
    

    pub fn read_to_string(&self) -> String {
        self.bytes().iter().map(|c| *c as char).collect::<String>()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.mmap[..]
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.mmap.to_vec()
    }

    pub fn file(&self) -> File {
        open_as_read(self.path.as_ref().as_ref())
    }

    pub fn mmap(&self) -> &Box<Mmap> {
       &self.mmap
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref().as_ref()
    }

    pub fn to_writer(&self) -> FileWriter {
        FileWriter::open(&self.path.as_ref())
    }
    
    pub fn hash_with<H: Digest>(&self) -> Output<H> {
        H::digest(&self.bytes())
    }
    pub fn hash(&self) -> Output<Sha3_256> {
        self.hash_with::<Sha3_256>()
    }

    fn find_inner(&self, i: &usize, byte: &u8, bytes: &[u8]) -> Option<usize>
  {
        if byte == &bytes[0] {
            let mut offset = 1;
            while offset < bytes.len() {
                if self.bytes()[i + offset] != bytes[offset] {
                    break;
                }
                offset += 1;
            }
            if offset == bytes.len() {
                Some(*i)
            } else {
                None
            }
        } else {
            None
        }
    }

    
      pub fn find_bytes(&self, bytes: &impl AsRef<[u8]>) -> Option<usize> {
           let bytes = bytes.as_ref();
           let mmap_bytes = self.bytes();
           mmap_bytes.into_par_iter().enumerate().find_map_first (
                |(i, byte)| {
                    self.find_inner(&i, &byte, bytes.as_ref())
                
                })}

        pub fn rfind_bytes(&self, bytes: &impl AsRef<[u8]>) -> Option<usize>
            {
                let bytes = bytes.as_ref();
                let mmap_bytes = self.bytes();
            mmap_bytes.into_par_iter().enumerate().find_map_last(
                |(i, byte)| {
                    self.find_inner(&i, &byte, bytes.as_ref())
                
                })}
                
            

        pub fn find_bytes_all(&self, bytes: &impl AsRef<[u8]>) -> Vec<usize>
    {
        let bytes = bytes.as_ref();
        let mmap_bytes = self.bytes();
            mmap_bytes.into_par_iter().enumerate().filter_map(
                |(i, byte)| {
                    self.find_inner(&i, &byte, bytes)
                
                }).collect::<Vec<usize>>()
            }

    pub fn find_bytes_nth(&self, bytes: &impl AsRef<[u8]>, n: usize) -> Option<usize>
            {
                self.find_bytes_all(bytes).get(n).map(|i| *i)

                

            }
    pub fn compare_files(file_path1: impl AsRef<Path>  + Send + Sync, file_path2: impl AsRef<Path>  + Send + Sync) -> bool {
        let file1_reader  = FileReader::open(&file_path1);
        let file2_reader = FileReader::open(&file_path2);
        file1_reader.hash() == file2_reader.hash()
    }
    
    pub fn compare_to(&self, file_path: impl AsRef<Path> + Send + Sync) -> bool {
        let file_reader = FileReader::open(&file_path);
        self.hash() == file_reader.hash()
    }

    pub fn compare_to_file(&self, file: &File) -> bool {
        let file_reader = FileReader::open_file(&file);
        self.hash() == file_reader.hash()
    }

    pub fn compare_hash<T: Digest>(&self, hash: &Output<T>) -> bool {
        self.hash_with::<T>() == *hash
    }

}

impl IntoIterator for FileReader {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes().to_vec().into_iter()
    }
}

impl IntoParallelIterator for FileReader {
    type Item = u8;
    type Iter = rayon::vec::IntoIter<Self::Item>;

    fn into_par_iter(self) -> Self::Iter {
        self.bytes().to_vec().into_par_iter()
    }
}

impl PartialEq for FileReader {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}



