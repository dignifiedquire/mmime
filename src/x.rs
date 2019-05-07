pub const MAILIMF_ERROR_FILE: libc::c_uint = 4;
pub const MAILIMF_ERROR_INVAL: libc::c_uint = 3;
pub const MAILIMF_ERROR_MEMORY: libc::c_uint = 2;
pub const MAILIMF_ERROR_PARSE: libc::c_uint = 1;
pub const MAILIMF_NO_ERROR: libc::c_uint = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}

pub use libc::pthread_mutex_t;
pub type size_t = __darwin_size_t;
pub type __int32_t = libc::c_int;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_pid_t = __int32_t;
pub type pid_t = __darwin_pid_t;
pub type time_t = __darwin_time_t;
pub type uint32_t = libc::c_uint;
pub type iconv_t = *mut libc::c_void;

extern "C" {
    pub fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    pub fn iconv_close(_cd: iconv_t) -> libc::c_int;
    pub fn free(_: *mut libc::c_void);
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    pub fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    pub fn __error() -> *mut libc::c_int;
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    pub fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    pub fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    pub fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    pub fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    pub fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    pub fn getpid() -> pid_t;
    pub fn random() -> libc::c_long;
    pub fn time(_: *mut time_t) -> time_t;
    pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
}
