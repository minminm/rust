use crate::ffi::OsString;
use crate::fmt;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
use core::mem::MaybeUninit;
use crate::io::Error;
use crate::path::{Path, PathBuf, };
use crate::sys::time::SystemTime;
use crate::sys::unsupported;
use crate::sys::ruxos::{abi, axerrno_to_error_kind};

pub use crate::os::ruxos::fs::FileType;
pub use crate::sys_common::fs::{copy, try_exists};


/// Owner has read permission.
const OWNER_READ: u16 = 0o400;
/// Owner has write permission.
#[allow(dead_code)]
const OWNER_WRITE: u16 = 0o200;
/// Owner has execute permission.
#[allow(dead_code)]
const OWNER_EXEC: u16 = 0o100;

/// Group has read permission.
const GROUP_READ: u16 = 0o40;
/// Group has write permission.
#[allow(dead_code)]
const GROUP_WRITE: u16 = 0o20;
/// Group has execute permission.
#[allow(dead_code)]
const GROUP_EXEC: u16 = 0o10;

/// Others have read permission.
const OTHER_READ: u16 = 0o4;
/// Others have write permission.
#[allow(dead_code)]
const OTHER_WRITE: u16 = 0o2;
/// Others have execute permission.
#[allow(dead_code)]
const OTHER_EXEC: u16 = 0o1;

#[derive(Debug)]
pub struct File {
    handle: usize,
}

impl File {
    fn new(handle: usize) -> Self {
        Self {
            handle,
        }
    }

    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        // let handle = (
        //     abi::sys_open(path.to_str().unwrap(), opts.to_flags())
        // )?;
        // Ok(File::new(handle))
        unsafe {
            match abi::sys_open(path.to_str().unwrap(), opts.to_flags()) {
                Ok(handle) => {
                    // assert_eq!(1, 99);
                    Ok(File::new(handle))
                },
                Err(e) => {
                    // assert_eq!(1, 101);
                    Err(io::Error::new(axerrno_to_error_kind(e), e.as_str()))
                }
            }
        }
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        // println!("###### file_attr\n");
        // Err(Error::from_raw_os_error(22))
        let mock_permissions = FilePermissions {
            mode: 0,
        };
        let mock_file_attr = FileAttr {
            mode: mock_permissions,
            ty: FileType::File,
            size: 0,
            blocks: 0,
        };
        Ok(mock_file_attr)
    }

    pub fn fsync(&self) -> io::Result<()> {
        // Err(Error::from_raw_os_error(22))
        abi_ret!(
            abi::sys_sync(self.handle)
        )
    }

    pub fn datasync(&self) -> io::Result<()> {
        self.fsync()
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        // Err(Error::from_raw_os_error(22))
        abi_ret!(
            abi::sys_truncate(self.handle, size)
        )
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            match abi::sys_read(self.handle, buf) {
                Ok(res) => Ok(res),
                Err(e) => Err(io::Error::new(axerrno_to_error_kind(e), e.as_str())),
            }
        }

        // unsafe { Ok(abi::sys_read(self.handle, buf)) }
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        crate::io::default_read_vectored(|buf| self.read(buf), bufs)
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        // self.0.read_at(buf, offset)
        unsafe { Ok(abi::sys_read_at(self.handle, buf, offset)) }
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        // crate::io::default_read_buf(|buf| self.read(buf), cursor)
        unsafe {
            let mut cursor = cursor;
            // let n = abi::sys_read(self.handle, MaybeUninit::slice_assume_init_mut(cursor.as_mut()));
            match abi::sys_read(self.handle, MaybeUninit::slice_assume_init_mut(cursor.as_mut())) {
                Ok(n) => {
                    cursor.advance(n);
                    Ok(())
                },
                Err(e) => {
                    Err(io::Error::new(axerrno_to_error_kind(e), e.as_str()))
                }
            }
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        // unsafe { Ok(abi::sys_write(self.handle, buf)) }
        unsafe {
            match abi::sys_write(self.handle, buf) {
                Ok(res) => Ok(res),
                Err(e) => Err(io::Error::new(axerrno_to_error_kind(e), e.as_str())),
            }
        }
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        // self.0.write_at(buf, offset)
        unsafe { Ok(abi::sys_write_at(self.handle, buf, offset)) }
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        crate::io::default_write_vectored(|buf| self.write(buf), bufs)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        abi_ret!(abi::sys_seek(self.handle, pos))
    }

    pub fn duplicate(&self) -> io::Result<File> {
        Err(Error::from_raw_os_error(22))
        // Ok(File::new(self.handle))
    }

    pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
        // Err(Error::from_raw_os_error(22))
        Ok(())
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        Err(Error::from_raw_os_error(22))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { abi::sys_close_file(self.handle) }
    }
}

/// Attributes for File or Directory.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct FileAttr {
    /// File permission mode.
    mode: FilePermissions,
    /// File type.
    ty: FileType,
    /// Total size, in bytes.
    size: u64,
    /// Number of 512B blocks allocated.
    blocks: u64,
}

pub struct ReadDir {
    root: PathBuf,
    handle: usize,
}

impl ReadDir {
    fn new(path: &Path, handle: usize) -> Self {
        Self { root: path.to_path_buf(), handle }
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.root, f)
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        Some(abi_ret!(abi::sys_read_dir_next(self.handle)?))
    }
}

impl Drop for ReadDir {
    fn drop(&mut self) {
        unsafe { abi::sys_close_dir(self.handle) }
    }
}

/// Directory entry.
#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct DirEntry {
    path: String,
    fname: String,
    ftype: FileType,
}

impl DirEntry {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn path(&self) -> PathBuf {
        self.path.as_str().into()
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn file_name(&self) -> OsString {
        self.fname.as_str().into()
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn metadata(&self) -> io::Result<FileAttr> {
        // panic!("Unsupported metadata()!")
        let mock_permissions = FilePermissions {
            mode: 0,
        };
        let mock_file_attr = FileAttr {
            mode: mock_permissions,
            ty: self.ftype,
            size: 0,
            blocks: 0,
        };
        Ok(mock_file_attr)
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(self.ftype)
    }
}

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }

    const F_READ:   u32 = 0x01;
    const F_WRITE:  u32 = 0x02;
    const F_APPEND: u32 = 0x04;
    const F_TRUNC:  u32 = 0x08;
    const F_CREATE: u32 = 0x10;
    const F_NEW:    u32 = 0x20;     /* for create_new */

    fn to_flags(&self) -> u32 {
        let mut flags = 0;
        if self.read { flags |= Self::F_READ };
        if self.write { flags |= Self::F_WRITE };
        if self.append { flags |= Self::F_APPEND };
        if self.truncate { flags |= Self::F_TRUNC };
        if self.create { flags |= Self::F_CREATE };
        if self.create_new { flags |= Self::F_NEW };
        flags
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

#[derive(Copy, Clone)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct FilePermissions {
    mode: u16,
}

impl FilePermissions {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn readonly(&self) -> bool {
        self.mode & (OWNER_READ|GROUP_READ|OTHER_READ) == 0
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn set_readonly(&mut self, readonly: bool) {
        if readonly {
            self.mode &= !(OWNER_READ|GROUP_READ|OTHER_READ);
        } else {
            self.mode |= OWNER_READ|GROUP_READ|OTHER_READ;
        }
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn mode(&self) -> u32 {
        self.mode as u32
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl PartialEq for FilePermissions {
    fn eq(&self, other: &FilePermissions) -> bool {
        self.mode == other.mode
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Eq for FilePermissions {}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for FilePermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FilePermissions {}", self.mode)
    }
}

#[derive(Debug)]
pub struct DirBuilder {}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, path: &Path) -> io::Result<()> {
        abi_ret!(
            abi::sys_mkdir(path.to_str().unwrap())
        )
    }
}

impl FileAttr {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn size(&self) -> u64 {
        self.size
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn perm(&self) -> FilePermissions {
        self.mode
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn file_type(&self) -> FileType {
        self.ty
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn modified(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn accessed(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn created(&self) -> io::Result<SystemTime> {
        unsupported()
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

pub fn readdir(path: &Path) -> io::Result<ReadDir> {
    unsafe {
        match abi::sys_read_dir(path.to_str().unwrap()) {
            Ok(handle) => Ok(ReadDir::new(path, handle)),
            Err(e) => Err(io::Error::new(axerrno_to_error_kind(e), e.as_str())),
        }
    }
    // let handle = unsafe { abi::sys_read_dir(path.to_str().unwrap()) };
    // Ok(ReadDir::new(path, handle))
}

pub fn unlink(path: &Path) -> io::Result<()> {
    abi_ret!(abi::sys_unlink(path.to_str().unwrap()))
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    abi_ret!(abi::sys_rename(old.to_str().unwrap(), new.to_str().unwrap()))
}

pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    unsupported()
}

pub fn rmdir(path: &Path) -> io::Result<()> {
    abi_ret!(
        abi::sys_rmdir(path.to_str().unwrap())
    )
}

fn remove_dir_all_recursive(path: &Path) -> io::Result<()> {
    let mut cnt : usize = 0;
    let mut path_vec = Vec::new();

    for child in readdir(path)? {
        let child = child?;
        if child.file_type()?.is_dir() {
            remove_dir_all_recursive(&child.path())?;
        } else {
            path_vec.push(child);
            // unlink(&child.path().as_ref())?;
        }

        cnt += 1;
    }
    for path in path_vec.iter() {
        unlink(&path.path().as_ref())?;
    }
    rmdir(path.as_ref())
}

pub fn remove_dir_all(path: &Path) -> io::Result<()> {
    remove_dir_all_recursive(path)
}

// pub fn try_exists(_path: &Path) -> io::Result<bool> {
//     unsupported()
// }

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    abi_ret!(abi::sys_stat(p.to_str().unwrap()))
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    unsafe {
        match abi::sys_canonicalize(p.to_str().unwrap()) {
            Ok(p) => Ok(PathBuf::from(p)),
            Err(e) => Err(io::Error::new(axerrno_to_error_kind(e), e.as_str())),
        }
    }
}

// pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
//     unsupported()
// }