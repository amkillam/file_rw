use zerocopy::AsBytes;
use memmap2::MmapMut;
use crate::{HashFn, PathRef};
use std::fs::File;

#[derive(AsBytes)]
#[repr(C)]
pub struct ZeroCopyMmapMut {
    mmap: MmapMut

}

impl ZeroCopyMmapMut {
    pub fn map_mut(file: &File) -> Self {
        let mmap = unsafe {
            MmapMut::map_mut(file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        };
        Self {
            mmap
        }
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.mmap[..]
    }

    pub fn mmap(&self) -> &MmapMut {
        &self.mmap
    }

    pub fn memmap_deref(&self) -> MmapMut {
        self.mmap
    }
}