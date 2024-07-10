#![allow(exported_private_dependencies)]

use core::alloc::Layout;
use core::time::Duration;
use core::sync::atomic::AtomicU32;
use core::net::SocketAddr;
use crate::sys::fs::DirEntry;
use crate::sys::fs::FileAttr;
use crate::io::SeekFrom;

use axerrno::AxError;

#[stable(feature = "rust1", since = "1.0.0")]
// pub use arceos_api::legacy::timespec;
pub use rust_std_api::timespec;

#[stable(feature = "rust1", since = "1.0.0")]
// pub use arceos_api::legacy::HandleType;
pub use rust_std_api::HandleType;

#[stable(feature = "rust1", since = "1.0.0")]
// pub use arceos_api::legacy::{SOCK_STREAM, SOCK_DGRAM};
pub use rust_std_api::{SOCK_STREAM, SOCK_DGRAM};

#[stable(feature = "rust1", since = "1.0.0")]
// pub use arceos_api::legacy::{CLOCK_MONOTONIC, CLOCK_REALTIME, NSEC_PER_SEC};
pub use rust_std_api::{CLOCK_MONOTONIC, CLOCK_REALTIME, NSEC_PER_SEC};

extern "Rust" {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_terminate() -> !;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_getcwd() -> Result<String, AxError>;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_chdir(path: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_alloc(layout: Layout) -> *mut u8;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_dealloc(ptr: *mut u8, layout: Layout);
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_console_write_bytes(bytes: &[u8]);
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_console_read_bytes(bytes: &mut [u8]) -> usize;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_console_write_byte(byte: u8);
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_console_read_byte() -> Option<u8>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_spawn2(
        func: Box<dyn FnOnce()>,
        prio: i32,
        stack_size: usize,
        core_id: isize,
    ) -> usize;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_join(handle: usize);
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_sleep(dur: Duration);
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_yield_now();
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_close_thread(handle: usize);

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_futex_wait(
        futex: &AtomicU32, expected: u32,
        timeout: Option<Duration>) -> bool;
    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_futex_wake(futex: &AtomicU32, count: i32);

    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_clock_gettime(_clock_id: u64, tp: *mut timespec) -> i32;

    //
    // For socket
    //
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_socket(family: i32, ty: i32) -> usize;

	/// bind a name to a socket
    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_bind(s: usize, addr: &SocketAddr);

	/// listen for connections on a socket
	///
	/// The `backlog` parameter defines the maximum length for the queue of pending
	/// connections. Currently, the `backlog` must be one.
    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_listen(s: usize, backlog: i32) -> i32;

    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_getsockname(s: usize) -> Result<SocketAddr, AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_accept(s: usize) -> Result<(usize, SocketAddr), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_recv(s: usize, buf: &mut [u8], flags: i32) -> usize;

    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_send(s: usize, buf: &[u8]) -> usize;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_connect(s: usize, addr: &SocketAddr);

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_recvfrom(s: usize, buf: &mut [u8], flags: i32)
        -> (usize, SocketAddr);

    #[stable(feature = "rust1", since = "1.0.0")]
	pub fn sys_sendto(s: usize, buf: &[u8], dst: &SocketAddr) -> usize;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_getaddrinfo(name: &str, port: u16) -> Result<Vec<SocketAddr>, AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_close_socket(handle: usize);

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_read_dir(path: &str) -> Result<usize, AxError>;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_read_dir_next(handle: usize) -> Option<Result<DirEntry, AxError>>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_mkdir(path: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_stat(path: &str) -> Result<FileAttr, AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_open(path: &str, opts: u32) -> Result<usize, AxError>;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_write(handle: usize, buf: &[u8]) -> Result<usize, AxError>;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_read(handle: usize, buf: &mut [u8]) -> Result<usize, AxError>;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_write_at(handle: usize, buf: &[u8], offset: u64) -> usize;
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_read_at(handle: usize, buf: &mut [u8], offset: u64) -> usize;
    
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_seek(handle: usize, pos: SeekFrom) -> Result<u64, AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_rmdir(path: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_unlink(path: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_close_file(handle: usize);
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_close_dir(handle: usize);

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_sync(handle: usize) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_truncate(handle: usize, size: u64) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_canonicalize(path: &str) -> Result<String, AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_rename(old: &str, new: &str) -> Result<(), AxError>;

    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn sys_debug(cnt: usize, msg: &str);

    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub fn sys_debug_direntry(d: &DirEntry);
    
}
