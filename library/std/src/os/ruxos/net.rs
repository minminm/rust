//! RuxOS-specific extensions to networking primitives.
#![stable(feature = "rust1", since = "1.0.0")]

use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::{net, sys};

#[stable(feature = "rust1", since = "1.0.0")]
// pub use arceos_api::net::AxTcpSocketHandle;
pub use rust_std_api::net::AxTcpSocketHandle;

#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRawTcpSocket {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_raw_socket(&self) -> &AxTcpSocketHandle;
}

#[stable(feature = "from_raw_os", since = "1.1.0")]
pub trait FromRawTcpSocket {
    #[stable(feature = "from_raw_os", since = "1.1.0")]
    unsafe fn from_raw_socket(sock: AxTcpSocketHandle) -> Self;
}

#[stable(feature = "into_raw_os", since = "1.4.0")]
pub trait IntoRawTcpSocket {
    #[stable(feature = "into_raw_os", since = "1.4.0")]
    fn into_raw_socket(self) -> AxTcpSocketHandle;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawTcpSocket for net::TcpStream {
    #[inline]
    fn as_raw_socket(&self) -> &AxTcpSocketHandle {
        self.as_inner().as_raw_socket()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawTcpSocket for net::TcpListener {
    #[inline]
    fn as_raw_socket(&self) -> &AxTcpSocketHandle {
        self.as_inner().as_raw_socket()
    }
}

#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawTcpSocket for net::TcpStream {
    #[inline]
    unsafe fn from_raw_socket(sock: AxTcpSocketHandle) -> net::TcpStream {
        net::TcpStream::from_inner(sys::net::TcpStream::from_raw_socket(sock))
    }
}

#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawTcpSocket for net::TcpListener {
    #[inline]
    unsafe fn from_raw_socket(sock: AxTcpSocketHandle) -> net::TcpListener {
        net::TcpListener::from_inner(sys::net::TcpListener::from_raw_socket(sock))
    }
}

#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawTcpSocket for net::TcpStream {
    #[inline]
    fn into_raw_socket(self) -> AxTcpSocketHandle {
        self.into_inner().into_raw_socket()
    }
}

#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawTcpSocket for net::TcpListener {
    #[inline]
    fn into_raw_socket(self) -> AxTcpSocketHandle {
        self.into_inner().into_raw_socket()
    }
}
