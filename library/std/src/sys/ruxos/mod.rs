//! System bindings for RuxOS
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for RuxOS.
//!
//! This is all super highly experimental and not actually intended for
//! wide/production use yet, it's still all in the experimental category. This
//! will likely change over time.
//!
//! Currently all functions here are basically stubs that immediately return
//! errors. The hope is that with a portability lint we can turn actually just
//! remove all this and just omit parts of the standard library if we're
//! compiling for wasm. That way it's a compile time error for something that's
//! guaranteed to be a runtime error!

#![allow(missing_docs, nonstandard_style, unsafe_op_in_unsafe_fn)]

// extern crate arceos_api;
extern crate rust_std_api;

use crate::os::raw::c_char;

// use arceos_api::{AxError, AxResult};
use rust_std_api::{AxError, AxResult};

// macro_rules! abi_ret {
//     ($expr:expr) => {
//         unsafe { $expr.or_else(|e| Err(io::Error::new(io::ErrorKind::Other, e.as_str()))) }
//     };
// }
macro_rules! abi_ret {
    ($expr:expr) => {
        unsafe { $expr.or_else(|e| Err(io::Error::new(axerrno_to_error_kind(e), e.as_str()))) }
    };
}

pub mod net;

pub mod thread;
#[path = "../unsupported/thread_parking.rs"]
pub mod thread_parking;

#[path = "../unix/cmath.rs"]
pub mod cmath;

pub mod alloc;
pub mod args;
#[path = "../unsupported/env.rs"]
pub mod env;
pub mod fs;
pub mod futex;
#[path = "../unsupported/io.rs"]
pub mod io;
pub mod memchr;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
pub mod time;

#[path = "../unix/locks"]
pub mod locks {
    mod futex_condvar;
    mod futex_mutex;
    mod futex_rwlock;
    pub(crate) use futex_condvar::Condvar;
    pub(crate) use futex_mutex::Mutex;
    pub(crate) use futex_rwlock::RwLock;
}

use crate::io::ErrorKind;
use crate::os::ruxos::abi;

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::const_io_error!(
        crate::io::ErrorKind::Unsupported,
        "operation not supported on RuxOS yet",
    )
}

pub fn abort_internal() -> ! {
    unsafe { abi::sys_terminate() }
}

// This function is needed by the panic runtime. The symbol is named in
// pre-link args for the target specification, so keep that in sync.
#[cfg(not(test))]
#[no_mangle]
// NB. used by both libunwind and libpanic_abort
pub extern "C" fn __rust_abort() {
    abort_internal();
}

// FIXME: just a workaround to test the system
pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(argc: isize, argv: *const *const u8, _sigpipe: u8) {
    args::init(argc, argv);
}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

#[cfg(not(test))]
#[no_mangle]
pub unsafe extern "C" fn runtime_entry(
    argc: i32,
    argv: *const *const c_char,
    env: *const *const c_char,
) {
    use crate::sys::ruxos::thread_local_dtor::run_dtors;
    extern "C" {
        fn main(argc: isize, argv: *const *const c_char) -> i32;
    }

    // initialize environment
    os::init_environment(env as *const *const i8);

    main(argc as isize, argv);

    run_dtors();
}

#[inline]
pub(crate) fn is_interrupted(errno: i32) -> bool {
    // errno == ErrorKind::Interrupted
    false
}

pub fn decode_error_kind(errno: i32) -> ErrorKind {
    if let Ok(err) = AxError::try_from(errno) {
        axerrno_to_error_kind(err)
    } else {
        ErrorKind::Uncategorized
    }
}

pub fn axerrno_to_error_kind(errno: AxError) -> ErrorKind {
    use AxError::*;
    
    match errno {
        AddrInUse => ErrorKind::AddrInUse,
        AlreadyExists => ErrorKind::AlreadyExists,
        ConnectionRefused => ErrorKind::ConnectionRefused,
        ConnectionReset => ErrorKind::ConnectionReset,
        DirectoryNotEmpty => ErrorKind::DirectoryNotEmpty,
        InvalidInput => ErrorKind::InvalidInput,
        InvalidData => ErrorKind::InvalidData,
        IsADirectory => ErrorKind::IsADirectory,
        NoMemory => ErrorKind::OutOfMemory,
        NotADirectory => ErrorKind::NotADirectory,
        NotConnected => ErrorKind::NotConnected,
        NotFound => ErrorKind::NotFound,
        PermissionDenied => ErrorKind::PermissionDenied,
        ResourceBusy => ErrorKind::ResourceBusy,
        StorageFull => ErrorKind::StorageFull,
        Unsupported => ErrorKind::Unsupported,
        UnexpectedEof => ErrorKind::UnexpectedEof,
        WriteZero => ErrorKind::WriteZero,
        WouldBlock => ErrorKind::WouldBlock,
        BadAddress | BadState | Io => ErrorKind::Other,
        _ => ErrorKind::Uncategorized,
    }
}

pub fn cvt<T>(t: AxResult<T>) -> crate::io::Result<T> {
    match t {
        Ok(t) => Ok(t),
        Err(e) => Err(crate::io::Error::from_raw_os_error(e.code())),
    }
}
