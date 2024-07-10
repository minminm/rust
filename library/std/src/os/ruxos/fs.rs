//! Unix-specific extensions to primitives in the [`std::fs`] module.
//!
//! [`std::fs`]: crate::fs

#![stable(feature = "rust1", since = "1.0.0")]

use crate::fs;
use crate::fs::Permissions;
use crate::hash::Hash;
use crate::io;
use crate::sys_common::AsInner;

/// Node (file/directory) type.
#[repr(u8)]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FileType {
    /// FIFO (named pipe)
    Fifo = 0o1,
    /// Character device
    CharDevice = 0o2,
    /// Directory
    Dir = 0o4,
    /// Block device
    BlockDevice = 0o6,
    /// Regular file
    File = 0o10,
    /// Symbolic link
    SymLink = 0o12,
    /// Socket
    Socket = 0o14,
}

/// Unix-specific extensions for [`fs::FileType`].
///
/// Adds support for special Unix file types such as block/character devices,
/// pipes, and sockets.
#[stable(feature = "file_type_ext", since = "1.5.0")]
pub trait FileTypeExt {
    /// Returns `true` if this file type is a block device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs;
    /// use std::os::unix::fs::FileTypeExt;
    /// use std::io;
    ///
    /// fn main() -> io::Result<()> {
    ///     let meta = fs::metadata("block_device_file")?;
    ///     let file_type = meta.file_type();
    ///     assert!(file_type.is_block_device());
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "file_type_ext", since = "1.5.0")]
    fn is_block_device(&self) -> bool;
    /// Returns `true` if this file type is a char device.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs;
    /// use std::os::unix::fs::FileTypeExt;
    /// use std::io;
    ///
    /// fn main() -> io::Result<()> {
    ///     let meta = fs::metadata("char_device_file")?;
    ///     let file_type = meta.file_type();
    ///     assert!(file_type.is_char_device());
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "file_type_ext", since = "1.5.0")]
    fn is_char_device(&self) -> bool;
    /// Returns `true` if this file type is a fifo.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs;
    /// use std::os::unix::fs::FileTypeExt;
    /// use std::io;
    ///
    /// fn main() -> io::Result<()> {
    ///     let meta = fs::metadata("fifo_file")?;
    ///     let file_type = meta.file_type();
    ///     assert!(file_type.is_fifo());
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "file_type_ext", since = "1.5.0")]
    fn is_fifo(&self) -> bool;
    /// Returns `true` if this file type is a socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs;
    /// use std::os::unix::fs::FileTypeExt;
    /// use std::io;
    ///
    /// fn main() -> io::Result<()> {
    ///     let meta = fs::metadata("unix.socket")?;
    ///     let file_type = meta.file_type();
    ///     assert!(file_type.is_socket());
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "file_type_ext", since = "1.5.0")]
    fn is_socket(&self) -> bool;
}

#[stable(feature = "file_type_ext", since = "1.5.0")]
impl FileTypeExt for fs::FileType {
    fn is_block_device(&self) -> bool {
        *(self.as_inner()) == FileType::BlockDevice
    }
    fn is_char_device(&self) -> bool {
        *(self.as_inner()) == FileType::CharDevice
    }
    fn is_fifo(&self) -> bool {
        *(self.as_inner()) == FileType::Fifo
    }
    fn is_socket(&self) -> bool {
        *(self.as_inner()) == FileType::Socket
    }
}

/// Unix-specific extensions to [`fs::Permissions`].
#[stable(feature = "fs_ext", since = "1.1.0")]
pub trait PermissionsExt {
    /// Returns the underlying raw `st_mode` bits that contain the standard
    /// Unix permissions for this file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::os::unix::fs::PermissionsExt;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::create("foo.txt")?;
    ///     let metadata = f.metadata()?;
    ///     let permissions = metadata.permissions();
    ///
    ///     println!("permissions: {:o}", permissions.mode());
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "fs_ext", since = "1.1.0")]
    fn mode(&self) -> u32;

    /// Sets the underlying raw bits for this set of permissions.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::os::unix::fs::PermissionsExt;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::create("foo.txt")?;
    ///     let metadata = f.metadata()?;
    ///     let mut permissions = metadata.permissions();
    ///
    ///     permissions.set_mode(0o644); // Read/write for owner and read for others.
    ///     assert_eq!(permissions.mode(), 0o644);
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "fs_ext", since = "1.1.0")]
    fn set_mode(&mut self, mode: u32);

    /// Creates a new instance of `Permissions` from the given set of Unix
    /// permission bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::Permissions;
    /// use std::os::unix::fs::PermissionsExt;
    ///
    /// // Read/write for owner and read for others.
    /// let permissions = Permissions::from_mode(0o644);
    /// assert_eq!(permissions.mode(), 0o644);
    /// ```
    #[stable(feature = "fs_ext", since = "1.1.0")]
    fn from_mode(mode: u32) -> Self;
}
#[stable(feature = "fs_ext", since = "1.1.0")]
impl PermissionsExt for Permissions {
    fn mode(&self) -> u32 {
        self.as_inner().mode()
    }

    fn set_mode(&mut self, _mode: u32) {
        panic!("no set_mode");
    }

    fn from_mode(_mode: u32) -> Permissions {
        panic!("no from_mode");
    }
}

impl FileType {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_dir(&self) -> bool {
        *self == Self::Dir
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_file(&self) -> bool {
        *self == Self::File
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_symlink(&self) -> bool {
        *self == Self::SymLink
    }
}

/// Unix-specific extensions to [`fs::File`].
#[stable(feature = "file_offset", since = "1.15.0")]
pub trait FileExt {
    /// Reads a number of bytes starting from a given offset.
    ///
    /// Returns the number of bytes read.
    ///
    /// The offset is relative to the start of the file and thus independent
    /// from the current cursor.
    ///
    /// The current file cursor is not affected by this function.
    ///
    /// Note that similar to [`File::read`], it is not an error to return with a
    /// short read.
    ///
    /// [`File::read`]: fs::File::read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io;
    /// use std::fs::File;
    /// use std::os::ruxos::prelude::FileExt;
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut buf = [0u8; 8];
    ///     let file = File::open("foo.txt")?;
    ///
    ///     // We now read 8 bytes from the offset 10.
    ///     let num_bytes_read = file.read_at(&mut buf, 10)?;
    ///     println!("read {num_bytes_read} bytes: {buf:?}");
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "file_offset", since = "1.15.0")]
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>;

    /// Writes a number of bytes starting from a given offset.
    ///
    /// Returns the number of bytes written.
    ///
    /// The offset is relative to the start of the file and thus independent
    /// from the current cursor.
    ///
    /// The current file cursor is not affected by this function.
    ///
    /// When writing beyond the end of the file, the file is appropriately
    /// extended and the intermediate bytes are initialized with the value 0.
    ///
    /// Note that similar to [`File::write`], it is not an error to return a
    /// short write.
    ///
    /// [`File::write`]: fs::File::write
    /// [`pwrite64`]: https://man7.org/linux/man-pages/man2/pwrite.2.html
    /// [bug]: https://man7.org/linux/man-pages/man2/pwrite.2.html#BUGS
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use std::io;
    /// use std::os::ruxos::prelude::FileExt;
    ///
    /// fn main() -> io::Result<()> {
    ///     let file = File::create("foo.txt")?;
    ///
    ///     // We now write at the offset 10.
    ///     file.write_at(b"sushi", 10)?;
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "file_offset", since = "1.15.0")]
    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>;
}

#[stable(feature = "file_offset", since = "1.15.0")]
impl FileExt for fs::File {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.as_inner().read_at(buf, offset)
    }

    fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.as_inner().write_at(buf, offset)
    }
}