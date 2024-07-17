use super::file::{open_as_read, open_as_write};
use memmap2::{Mmap, MmapMut};
use std::{io::Write, path::Path};

/// Opens the file at the provided path with read permissions, and returns its memory-mapped contents.
pub fn get_mmap_read(path: impl AsRef<Path> + Send + Sync) -> std::io::Result<Mmap> {
    let file = open_as_read(path.as_ref())?;
    unsafe { Mmap::map(&file) }
}

/// Opens the file at the provided path with write permissions, and returns its memory-mapped contents.
/// The file's length will be set to the provided length.
pub fn get_mmap_write(path: impl AsRef<Path> + Send + Sync, len: u64) -> std::io::Result<MmapMut> {
    let file = open_as_write(path.as_ref())?;
    file.set_len(len)?;
    unsafe { MmapMut::map_mut(&file) }
}

/// Read the file contents and copy to the provided buffer.
/// Only bytes up to the buffer's length will be copied.
/// Returns the number of bytes read.
pub fn read_to_buf(
    path: impl AsRef<Path> + Send + Sync,
    mut slice: impl AsMut<[u8]>,
) -> std::io::Result<usize> {
    let slice = slice.as_mut();
    let file = open_as_read(path.as_ref())?;
    let mmap = unsafe { Mmap::map(&file)? };
    let len = std::cmp::min(mmap.len(), slice.len());
    slice[..len].copy_from_slice(&mmap[..len]);
    Ok(len)
}

/// Reads the file contents and outputs a vector containing them.
pub fn read_to_vec(path: impl AsRef<Path> + Send + Sync) -> std::io::Result<Vec<u8>> {
    let mmap = get_mmap_read(path)?;
    Ok(mmap.to_vec())
}

/// Reads the file contents and outputs them as a string.
pub fn read_to_string(path: impl AsRef<Path> + Send + Sync) -> std::io::Result<String> {
    let mmap = get_mmap_read(path)?;
    Ok(String::from_utf8_lossy(&mmap).to_string())
}

/// Reads the file contents and creates return from the output.
/// Return type must implement From<Vec<u8>>.
pub fn read_to<T: From<Vec<u8>>>(path: impl AsRef<Path> + Send + Sync) -> std::io::Result<T> {
    let mmap = get_mmap_read(path)?;
    Ok(T::from(mmap.to_vec()))
}
/// Read the file contents into the provided reader.
/// Returns the number of bytes read.
pub fn read_to_writer<W: Write>(
    path: impl AsRef<Path> + Send + Sync,
    mut writer: W,
) -> std::io::Result<()> {
    let mmap = get_mmap_read(path)?;
    writer.write_all(&mmap[..])
}

/// Overwrites the provided file with the contents of the provided buffer.
pub fn overwrite(
    path: impl AsRef<Path> + Send + Sync,
    slice: impl AsRef<[u8]>,
) -> std::io::Result<()> {
    let slice = slice.as_ref();
    let mut mmap = get_mmap_write(path, slice.len() as u64)?;
    mmap[..].copy_from_slice(slice);
    Ok(())
}

/// Append the provided file with the contents of the provided buffer.
pub fn append(
    path: impl AsRef<Path> + Send + Sync,
    slice: impl AsRef<[u8]>,
) -> std::io::Result<()> {
    let slice = slice.as_ref();
    let file = open_as_write(path.as_ref())?;
    let file_len = file.metadata()?.len();
    file.set_len(file_len + slice.len() as u64)?;
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };
    mmap[file_len as usize..].copy_from_slice(slice);
    Ok(())
}
