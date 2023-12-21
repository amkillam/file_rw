use ouroboros::self_referencing;
use zerocopy::AsBytes;
use crate::{FileReader, HashFn, PathRef, file::{
    open_as_append,
    open_as_write,
}, write::ZeroCopyMmapMut};

#[self_referencing]
pub struct FileWriter <P:PathRef> {
    mmap: Box<ZeroCopyMmapMut>,
    path: P,

    #[borrows(mmap)]
    mmap_ref: &'this mut Box<ZeroCopyMmapMut>,

    #[borrows(path)]
    path_ref: &'this P
}

impl<B: AsBytes, P: PathRef> FileWriter <P>{
    pub fn open(path: &P) -> Self {
        let file = open_as_write(&path);
        let mut mmap = Box::new(unsafe {
            ZeroCopyMmapMut::map_mut(&file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        });
        let mmap_ref = &mut mmap;
        let path_ref = path;
        let path = *path;
        Self {
            mmap,
            path,
            mmap_ref,
            path_ref
        }
    }

    pub fn open_append(path: &P) -> Self {
        let file = open_as_append(&path);
        let mut mmap = Box::new(unsafe {
            ZeroCopyMmapMut::map_mut(&file)
                .unwrap_or_else(|err| panic!("Could not mmap file. Error: {}", err))
        });
        let mmap_ref = &mut mmap;
        let path_ref = path;
        let path = *path;
        Self {
            mmap,
            path,
            mmap_ref,
            path_ref
        }
    }

    pub fn write(&self, bytes: &B) {
        self.mmap_ref.bytes_mut().clone_from_slice(bytes.as_bytes())
    }

    pub fn replace(&self, bytes: &B, offset: usize) {
        let bytes = bytes.as_bytes();
        self.mmap_ref[offset..offset + bytes.len()].clone_from_slice(bytes);
    }

    pub fn find_replace (
        &self, 
        find: &B,
        replace: &B
    ) -> Self
    {
        let find = find.as_bytes();
        let replace = replace.as_bytes();
        let mut offset = FileReader::find(&self.mmap_ref, &find).unwrap_or_else(
            return self
        );
        self.mmap_ref[offset..offset + replace.len()].clone_from_slice(replace);
        Self
    }

    pub fn find_replace_nth(&self, find: &B, replace: &B, n: usize) -> Self {
        let find = find.as_bytes();
        let replace = replace.as_bytes();
        let mut offset = FileReader::find_nth(&self.mmap_ref, &find, n)?;
        self.mmap_ref[offset..offset + replace.len()].clone_from_slice(replace);
        Self
    }
    

    pub fn find_replace_all(&self, find: &B, replace: &B) -> Self {
        let find = find.as_bytes();
        let replace = replace.as_bytes();
        FileReader::find_all(&self.mmap_ref, &find).unwrap_or_else(
            return Self
        ).par_iter().for_each(|offset| {
            self.mmap_ref[offset..offset + replace.len()].clone_from_slice(replace);
        });

        Self

    }


    pub fn path(&self) -> &P {
        &self.path
    }

    pub fn path_deref(&self) -> P {
        self.path
    }

    pub fn zerocopy_mmap(&self) -> &mut Box<ZeroCopyMmapMut> {
        self.mmap_ref
    }

    pub fn zerocopy_mmap_deref(&self) -> ZeroCopyMmapMut {
        self.mmap
    }

    pub fn to_reader(&self) -> FileReader<P> {
        FileReader::new(&self.path)
    }

    
}
