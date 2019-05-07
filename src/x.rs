use crate::clist::*;
use crate::mailmime_types::*;
use crate::mailmime_types_helper::*;

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

pub use libc::{
    atoi, calloc, close, closedir, exit, fclose, fgets, fopen, fread, free, fseek, fstat, ftell,
    fwrite, gmtime, gmtime_r, isalpha, isdigit, localtime, localtime_r, malloc, memcmp, memcpy,
    memmove, memset, mkdir, open, opendir, printf, pthread_mutex_t, read, readdir, realloc, remove,
    sleep, snprintf, sprintf, sscanf, strcasecmp, strcat, strchr, strcmp, strcpy, strcspn, strdup,
    strlen, strncasecmp, strncmp, strncpy, strrchr, strspn, strstr, strtol, system, time,
    tolower as __tolower, toupper as __toupper, usleep, write,
};

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
    pub fn __error() -> *mut libc::c_int;
    pub fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    pub fn getpid() -> pid_t;
    pub fn random() -> libc::c_long;
    pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn mailprivacy_prepare_mime(mut mime: *mut mailmime) {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    match (*mime).mm_type {
        1 => {
            if !(*mime).mm_data.mm_single.is_null() {
                prepare_mime_single(mime);
            }
        }
        2 => {
            cur = (*(*mime).mm_data.mm_multipart.mm_mp_list).first;
            while !cur.is_null() {
                let mut child: *mut mailmime = 0 as *mut mailmime;
                child = (if !cur.is_null() {
                    (*cur).data
                } else {
                    0 as *mut libc::c_void
                }) as *mut mailmime;
                mailprivacy_prepare_mime(child);
                cur = if !cur.is_null() {
                    (*cur).next
                } else {
                    0 as *mut clistcell
                }
            }
        }
        3 => {
            if !(*mime).mm_data.mm_message.mm_msg_mime.is_null() {
                mailprivacy_prepare_mime((*mime).mm_data.mm_message.mm_msg_mime);
            }
        }
        _ => {}
    };
}

unsafe extern "C" fn prepare_mime_single(mut mime: *mut mailmime) {
    let mut single_fields: mailmime_single_fields = mailmime_single_fields {
        fld_content: 0 as *mut mailmime_content,
        fld_content_charset: 0 as *mut libc::c_char,
        fld_content_boundary: 0 as *mut libc::c_char,
        fld_content_name: 0 as *mut libc::c_char,
        fld_encoding: 0 as *mut mailmime_mechanism,
        fld_id: 0 as *mut libc::c_char,
        fld_description: 0 as *mut libc::c_char,
        fld_version: 0,
        fld_disposition: 0 as *mut mailmime_disposition,
        fld_disposition_filename: 0 as *mut libc::c_char,
        fld_disposition_creation_date: 0 as *mut libc::c_char,
        fld_disposition_modification_date: 0 as *mut libc::c_char,
        fld_disposition_read_date: 0 as *mut libc::c_char,
        fld_disposition_size: 0,
        fld_language: 0 as *mut mailmime_language,
        fld_location: 0 as *mut libc::c_char,
    };
    let mut encoding: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if !(*mime).mm_mime_fields.is_null() {
        mailmime_single_fields_init(
            &mut single_fields,
            (*mime).mm_mime_fields,
            (*mime).mm_content_type,
        );
        if !single_fields.fld_encoding.is_null() {
            encoding = (*single_fields.fld_encoding).enc_type;
            match encoding {
                2 | 1 | 3 => {
                    (*single_fields.fld_encoding).enc_type =
                        MAILMIME_MECHANISM_QUOTED_PRINTABLE as libc::c_int
                }
                _ => {}
            }
        } else {
            let mut mechanism: *mut mailmime_mechanism = 0 as *mut mailmime_mechanism;
            let mut field: *mut mailmime_field = 0 as *mut mailmime_field;
            mechanism = mailmime_mechanism_new(
                MAILMIME_MECHANISM_QUOTED_PRINTABLE as libc::c_int,
                0 as *mut libc::c_char,
            );
            if mechanism.is_null() {
                return;
            }
            field = mailmime_field_new(
                MAILMIME_FIELD_TRANSFER_ENCODING as libc::c_int,
                0 as *mut mailmime_content,
                mechanism,
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
                0i32 as uint32_t,
                0 as *mut mailmime_disposition,
                0 as *mut mailmime_language,
                0 as *mut libc::c_char,
            );
            if field.is_null() {
                mailmime_mechanism_free(mechanism);
                return;
            }
            r = clist_insert_after(
                (*(*mime).mm_mime_fields).fld_list,
                (*(*(*mime).mm_mime_fields).fld_list).last,
                field as *mut libc::c_void,
            );
            if r < 0i32 {
                mailmime_field_free(field);
                return;
            }
        }
    }
    if (*mime).mm_type == MAILMIME_SINGLE as libc::c_int {
        match (*(*mime).mm_data.mm_single).dt_encoding {
            2 | 1 | 3 => {
                (*(*mime).mm_data.mm_single).dt_encoding =
                    MAILMIME_MECHANISM_QUOTED_PRINTABLE as libc::c_int;
                (*(*mime).mm_data.mm_single).dt_encoded = 0i32
            }
            _ => {}
        }
    };
}
