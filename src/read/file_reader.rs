use ouroboros::self_referencing;
use zerocopy::AsBytes;
use crate::{FileWriter, HashFn, PathRef};
use memmap2::Mmap;
use digest::Output;
use sha3::Sha3_256;
use crate::file::open_as_read;
use crate::read::ZeroCopyMmap;
use std::fs::File;

#[self_referencing]
pub struct FileReader <P: PathRef> {
    mmap: Box<ZeroCopyMmap>,
    path: P,
    file: File,

    #[borrows(mmap)]
    mmap_ref: &'this Box<ZeroCopyMmap>,
    
    #[borrows(path)]
    path_ref: &'this P,

    #[borrows(file)]
    file_ref: &'this File,
}

impl <B:AsBytes, P: PathRef> FileReader <P>{
    pub fn open(path: &P) -> Self {
        let file = open_as_read(&path);
        let mmap = Box::new(unsafe {
            ZeroCopyMmap::map(&file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        });
        let mmap_ref = &mmap;
        Self {
            mmap,
            path: *path,
            file,
            mmap_ref: &mmap,
            path_ref: path,
            file_ref: &file
        }
    }

    pub fn read_to_string(&self) -> String {
        self.mmap.bytes().iter().map(|c| *c as char).collect::<String>()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.mmap.bytes()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.mmap.to_vec()
    }

    pub fn hash<T: HashFn>(&self) -> Output<T> {
        HashFn::compute(&self.mmap[..])
    }

    pub fn hash(&self) -> Output<Sha3_256> {
        self.hash::<Sha3_256>()
    }

    pub fn path(&self) -> &P {
        &self.path
    }

    pub fn path_deref(&self) -> P {
        self.path
    }

    pub fn mmap(&self) -> &Mmap {
        &self.mmap
    }

    pub fn mmap_deref(&self) -> Mmap {
        self.mmap
    }

    pub fn to_writer(&self) -> FileWriter<P> {
        FileWriter::new(&self.path)
    }

    fn find_inner(&self, i: &usize, byte: &u8, bytes: &B) -> Option<usize>
  {
        if byte == bytes[0] {
            let mut offset = 1;
            while offset < bytes.len() {
                if self.mmap_ref.bytes()[i + offset] != bytes[offset] {
                    break;
                }
                offset += 1;
            }
            if offset == bytes.len() {
                Some(i)
            } else {
                None
            }
        } else {
            None
        }
    }
      pub fn find(&self, bytes: &B) -> Option<usize> {
        let bytes = bytes.as_bytes();

            self.mmap_ref.bytes()[..].par_iter().enumerate().find_map(
                |(i, byte)| {
                    self.find_inner(&i, &byte, &bytes)
                
                })}

        pub fn rfind(&self, bytes: &B) -> Option<usize>
            {
                let bytes = bytes.as_bytes();
            self.mmap_ref.bytes()[..].par_iter().enumerate().rfind_map(
                |(i, byte)| {
                    self.find_inner(&i, &byte, &bytes)
                
                })}
                
            

        pub fn find_all(&self, bytes: &B) -> Option<&Vec<usize>>
    {
            let bytes = bytes.as_bytes();
            self.mmap_ref[..].par_iter().enumerate().map(
                |(i, byte)| {
                    self.find_inner(&i, &byte, &bytes)
                
                }).collect()}

        pub fn find_nth(&self, bytes: &B, n: usize) -> Option<usize>
            {
                let bytes = bytes.as_bytes();
                let mut counter = 0;
            self.mmap_ref.bytes()[..].par_iter().enumerate().find_map(
                |(i, byte)| {
                    let result = self.find_inner(&i, &byte, &bytes);
                    if result.is_some() {
                        counter += 1;
                        if counter == n {
                            result
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                
                })}

    pub fn equivalent_to(file1: &P, file2: &P) -> bool {
        let file1_reader = FileReader::new(&file1);
        let file2_reader = FileReader::new(&file2);
        file1_reader.hash() == file2_reader.hash()
    }
    
    pub fn equivalent_to(&self, file: &P) -> bool {
        let file_reader = FileReader::new(&file);
        self.hash() == file_reader.hash()
    }

    pub fn equivalent_to<T: HashFn>(hash: &Output<T>, file: &P) -> bool {
        let file_reader = FileReader::new(&file);
        hash == &file_reader.hash()
    }

}