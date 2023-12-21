use std::fs::File;
use zerocopy::AsBytes;
use delegate::delegate;
use memmap2::Mmap;

#[derive(AsBytes)]
#[repr(C)]
pub struct ZeroCopyMmap {
    mmap: Mmap
}

impl ZeroCopyMmap {
    pub fn map(file: &File) -> Self {
        let mmap = unsafe {
            Mmap::map(file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        };
        Self {
            mmap
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.mmap.to_vec()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.as_bytes()
    }

    pub fn mmap(&self) -> &mut Mmap {
        &mut self.mmap
    }

    pub fn mmap_deref(&self) -> Mmap {
        self.mmap
    }

}