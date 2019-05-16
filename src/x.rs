pub use crate::other::*;

pub type size_t = libc::size_t;
pub type __int32_t = libc::c_int;
pub type pid_t = libc::pid_t;
pub type time_t = libc::time_t;
pub type uint32_t = libc::c_uint;
pub type dev_t = libc::dev_t;
pub type blkcnt_t = libc::blkcnt_t;
pub type blksize_t = libc::blksize_t;
pub type gid_t = libc::gid_t;
pub type mode_t = libc::mode_t;
pub type nlink_t = libc::uint16_t;
pub type off_t = libc::off_t;
pub type uid_t = libc::uid_t;

extern "C" {
    pub fn getpid() -> pid_t;
    pub fn random() -> libc::c_long;
    pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
}
