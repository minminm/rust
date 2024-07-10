use crate::os::ruxos::ffi::OsStringExt;
use crate::os::ruxos::ffi::os_str::OsStrExt;

use crate::collections::HashMap;
use crate::error::Error as StdError;
use crate::ffi::{CStr, OsStr, OsString};
use crate::fmt;
use crate::io;
use crate::iter;
use crate::marker::PhantomData;
use crate::path::{self, PathBuf};
use crate::slice;
use crate::sync::Mutex;
use crate::sys::{cvt, memchr, unsupported};
use crate::vec;

const PATH_SEPARATOR: u8 = b':';

pub fn errno() -> i32 {
    0
}

pub fn error_string(errno: i32) -> String {
    // if let Ok(e) = arceos_api::AxError::try_from(errno) {
    if let Ok(e) = rust_std_api::AxError::try_from(errno) {
            e.as_str().to_string()
    } else {
        format!("Unknown error: {}", errno)
    }
}

pub fn getcwd() -> io::Result<PathBuf> {
    // cvt(arceos_api::fs::ax_current_dir()).map(Into::into)
    cvt(rust_std_api::fs::current_dir()).map(Into::into)
}

pub fn chdir(path: &path::Path) -> io::Result<()> {
    let path_str = path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8 characters")
    })?;
    // cvt(arceos_api::fs::ax_set_current_dir(path_str))
    cvt(rust_std_api::fs::set_current_dir(path_str))
}

// pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

// pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
//     panic!("unsupported")
// }

pub struct SplitPaths<'a> {
    iter: iter::Map<slice::Split<'a, u8, fn(&u8) -> bool>, fn(&'a [u8]) -> PathBuf>,
}

pub fn split_paths(unparsed: &OsStr) -> SplitPaths<'_> {
    fn bytes_to_path(b: &[u8]) -> PathBuf {
        PathBuf::from(<OsStr as OsStrExt>::from_bytes(b))
    }
    fn is_separator(b: &u8) -> bool {
        *b == PATH_SEPARATOR
    }
    let unparsed = unparsed.as_bytes();
    SplitPaths {
        iter: unparsed
            .split(is_separator as fn(&u8) -> bool)
            .map(bytes_to_path as fn(&[u8]) -> PathBuf),
    }
}


impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.iter.next()
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

// pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
// where
//     I: Iterator<Item = T>,
//     T: AsRef<OsStr>,
// {
//     Err(JoinPathsError)
// }

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    let mut joined = Vec::new();

    for (i, path) in paths.enumerate() {
        let path = path.as_ref().as_bytes();
        if i > 0 {
            joined.push(PATH_SEPARATOR)
        }
        if path.contains(&PATH_SEPARATOR) {
            return Err(JoinPathsError);
        }
        joined.extend_from_slice(path);
    }
    Ok(OsStringExt::from_vec(joined))
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on ruxos yet".fmt(f)
    }
}

impl StdError for JoinPathsError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "not supported on ruxos yet"
    }
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

static mut ENV: Option<Mutex<HashMap<OsString, OsString>>> = None;

pub fn init_environment(env: *const *const i8) {
    unsafe {
        ENV = Some(Mutex::new(HashMap::new()));

        if env.is_null() {
            return;
        }

        let mut guard = ENV.as_ref().unwrap().lock().unwrap();
        let mut environ = env;
        while !(*environ).is_null() {
            if let Some((key, value)) = parse(CStr::from_ptr(*environ).to_bytes()) {
                guard.insert(key, value);
            }
            environ = environ.add(1);
        }
    }

    fn parse(input: &[u8]) -> Option<(OsString, OsString)> {
        // Strategy (copied from glibc): Variable name and value are separated
        // by an ASCII equals sign '='. Since a variable name must not be
        // empty, allow variable names starting with an equals sign. Skip all
        // malformed lines.
        if input.is_empty() {
            return None;
        }
        let pos = memchr::memchr(b'=', &input[1..]).map(|p| p + 1);
        pos.map(|p| {
            (
                OsStringExt::from_vec(input[..p].to_vec()),
                OsStringExt::from_vec(input[p + 1..].to_vec()),
            )
        })
    }
}

pub struct Env {
    iter: vec::IntoIter<(OsString, OsString)>,
}

pub struct EnvStrDebug<'a> {
    slice: &'a [(OsString, OsString)],
}

impl fmt::Debug for EnvStrDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { slice } = self;
        f.debug_list()
            .entries(slice.iter().map(|(a, b)| (a.to_str().unwrap(), b.to_str().unwrap())))
            .finish()
    }
}

impl Env {
    pub fn str_debug(&self) -> impl fmt::Debug + '_ {
        let Self { iter } = self;
        EnvStrDebug { slice: iter.as_slice() }
    }
}

impl fmt::Debug for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { iter } = self;
        f.debug_list().entries(iter.as_slice()).finish()
    }
}

impl !Send for Env {}
impl !Sync for Env {}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Returns a vector of (variable, value) byte-vector pairs for all the
/// environment variables of the current process.
pub fn env() -> Env {
    unsafe {
        let guard = ENV.as_ref().unwrap().lock().unwrap();
        let mut result = Vec::new();

        for (key, value) in guard.iter() {
            result.push((key.clone(), value.clone()));
        }

        return Env { iter: result.into_iter() };
    }
}

pub fn getenv(k: &OsStr) -> Option<OsString> {
    unsafe { ENV.as_ref().unwrap().lock().unwrap().get_mut(k).cloned() }
}

pub fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> {
    unsafe {
        let (k, v) = (k.to_owned(), v.to_owned());
        ENV.as_ref().unwrap().lock().unwrap().insert(k, v);
    }
    Ok(())
}

pub fn unsetenv(k: &OsStr) -> io::Result<()> {
    unsafe {
        ENV.as_ref().unwrap().lock().unwrap().remove(k);
    }
    Ok(())
}

pub fn temp_dir() -> PathBuf {
    // panic!("no filesystem on ruxos")
    crate::env::var_os("TMPDIR").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("/tmp")
    })
}

pub fn home_dir() -> Option<PathBuf> {
    return crate::env::var_os("HOME").or_else(|| {
        None
    }).map(PathBuf::from);
}

pub fn exit(_code: i32) -> ! {
    // arceos_api::sys::ax_terminate();
    rust_std_api::terminate();
}

pub fn getpid() -> u32 {
    panic!("os::getpid!");
}
