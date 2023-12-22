use std::{fs::File, path::Path};

use crate::{
    file::{open_as_append, open_as_write},
    FileReader, PathRef,
};
use filepath::FilePath;
use memmap2::MmapMut;

pub struct FileWriter {
    mmap: Box<MmapMut>,
    path: Box<dyn PathRef + Send + Sync>,
}

impl FileWriter {
    fn new<'a>(file: &File, path: impl AsRef<Path> + Send + Sync + 'static) -> Self {
        let mmap = Box::new(unsafe {
            MmapMut::map_mut(file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        });

        Self {
            mmap,
            path: Box::new(path),
        }
    }

    pub fn open_file(file: File) -> Self {
        let path = file
            .path()
            .unwrap_or_else(|err| panic!("Could not get path of writer file. Error: {}", err));
        Self::new(&file, path)
    }

    pub fn open(path: impl AsRef<Path> + Send + Sync) -> Self {
        let file = open_as_write(path.as_ref());
        FileWriter::open_file(file)
    }

    pub fn open_append(path: impl AsRef<Path> + Send + Sync) -> Self {
        let file = open_as_append(path.as_ref());
        FileWriter::open_file(file)
    }

    pub fn write(&mut self, bytes: &impl AsRef<[u8]>) -> &Self {
        self.mmap[..].clone_from_slice(bytes.as_ref());
        self
    }

    pub fn replace(&mut self, bytes: &impl AsRef<[u8]>, offset: usize) -> &Self {
        let bytes = bytes.as_ref();
        self.mmap[offset..offset + bytes.len()].clone_from_slice(bytes);
        self
    }

    pub fn find_replace(&mut self, find: &impl AsRef<[u8]>, replace: &impl AsRef<[u8]>) -> &Self {
        let find = find.as_ref();
        let replace = replace.as_ref();
        let file_reader = FileReader::open(&*(self.path.as_ref().as_ref()));
        let offset = file_reader.find_bytes(&find);
        match offset {
            Some(offset) => {
                self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
            }
            None => (),
        }
        self
    }

    pub fn find_replace_nth(
        &mut self,
        find: &impl AsRef<[u8]>,
        replace: &impl AsRef<[u8]>,
        n: usize,
    ) -> &Self {
        let replace = replace.as_ref();
        let file_reader = FileReader::open(self.path.as_ref());
        let offset = file_reader.find_bytes_nth(&find, n);
        match offset {
            Some(offset) => {
                self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
            }
            None => (),
        }
        self
    }

    pub fn find_replace_all(
        &mut self,
        find: &impl AsRef<[u8]>,
        replace: &impl AsRef<[u8]>,
    ) -> &Self {
        let replace = &replace.as_ref();
        let file_reader = FileReader::open(self.path.as_ref());
        let find_results = file_reader.find_bytes_all(find);
        for offset in find_results {
            let _ = &mut self.mmap[offset..offset + replace.len()].clone_from_slice(replace);
        }

        self
    }

    pub fn file(&mut self) -> File {
        open_as_write(self.path.as_ref().as_ref())
    }

    pub fn path(&mut self) -> &Box<dyn PathRef + Send + Sync> {
        &self.path
    }

    pub fn mmap(&mut self) -> &mut Box<MmapMut> {
        &mut self.mmap
    }

    pub fn to_reader(&mut self) -> FileReader {
        FileReader::open(self.path.as_ref())
    }
}
