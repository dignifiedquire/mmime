pub use crate::other::*;

pub type size_t = libc::size_t;
pub type __int32_t = libc::c_int;
pub type pid_t = libc::pid_t;
pub type time_t = libc::time_t;
pub type uint32_t = libc::c_uint;
pub type iconv_t = *mut libc::c_void;
pub type dev_t = libc::dev_t;
pub type blkcnt_t = libc::blkcnt_t;
pub type blksize_t = libc::blksize_t;
pub type gid_t = libc::gid_t;
pub type mode_t = libc::mode_t;
pub type nlink_t = libc::uint16_t;
pub type off_t = libc::off_t;
pub type uid_t = libc::uid_t;

extern "C" {
    pub fn munmap(_: *mut libc::c_void, _: size_t) -> libc::c_int;
    pub fn mmap(
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: off_t,
    ) -> *mut libc::c_void;

    pub fn iconv_close(_cd: iconv_t) -> libc::c_int;
    pub fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    pub fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    pub fn getpid() -> pid_t;
    pub fn random() -> libc::c_long;
    pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
}
