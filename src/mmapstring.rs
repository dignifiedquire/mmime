use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use libc;

use crate::other::*;

pub const TMPDIR: &'static str = "/tmp";

lazy_static! {
    static ref hashtable: Mutex<HashMap<Vec<u8>, Box<MMAPString>>> = Mutex::new(HashMap::new());
}

static mut mmap_string_ceil: size_t = (8i32 * 1024i32 * 1024i32) as size_t;

pub struct MMAPString {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub allocated_len: size_t,
    pub fd: libc::c_int,
    pub mmapped_size: size_t,
}

unsafe impl Send for MMAPString {}

impl Default for MMAPString {
    fn default() -> Self {
        MMAPString {
            str_0: std::ptr::null_mut(),
            len: 0,
            allocated_len: 0,
            fd: 0,
            mmapped_size: 0,
        }
    }
}

impl Drop for MMAPString {
    fn drop(&mut self) {
        unsafe {
            free(self.str_0 as *mut _);
        }
    }
}

pub unsafe fn mmap_string_new(mut init: *const libc::c_char) -> *mut MMAPString {
    let string = mmap_string_sized_new(if !init.is_null() { strlen(init) + 2 } else { 2 });
    if !init.is_null() {
        mmap_string_append(string, init);
    }
    return string;
}

pub unsafe fn mmap_string_append(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
) -> *mut MMAPString {
    return mmap_string_insert_len(string, (*string).len, val, strlen(val));
}

pub unsafe fn mmap_string_insert_len(
    mut string: *mut MMAPString,
    mut pos: size_t,
    mut val: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    if mmap_string_maybe_expand(string, len).is_null() {
        return std::ptr::null_mut();
    }
    if pos < (*string).len {
        memmove(
            (*string).str_0.offset(pos as isize).offset(len as isize) as *mut libc::c_void,
            (*string).str_0.offset(pos as isize) as *const libc::c_void,
            (*string).len.wrapping_sub(pos),
        );
    }
    memmove(
        (*string).str_0.offset(pos as isize) as *mut libc::c_void,
        val as *const libc::c_void,
        len,
    );
    (*string).len = ((*string).len as libc::size_t).wrapping_add(len) as size_t as size_t;
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    string
}

unsafe fn mmap_string_maybe_expand(
    mut string: *mut MMAPString,
    mut len: size_t,
) -> *mut MMAPString {
    if (*string).len.wrapping_add(len) >= (*string).allocated_len {
        let mut old_size: size_t = 0;
        let mut newstring: *mut MMAPString = 0 as *mut MMAPString;
        old_size = (*string).allocated_len;
        (*string).allocated_len = nearest_power(
            1i32 as size_t,
            (*string)
                .len
                .wrapping_add(len)
                .wrapping_add(1i32 as libc::size_t),
        );
        newstring = mmap_string_realloc_memory(string);
        if newstring.is_null() {
            (*string).allocated_len = old_size
        }
        return newstring;
    }
    return string;
}

unsafe fn mmap_string_realloc_memory(mut string: *mut MMAPString) -> *mut MMAPString {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = realloc(
        (*string).str_0 as *mut libc::c_void,
        (*string).allocated_len,
    ) as *mut libc::c_char;
    if tmp.is_null() {
        string = 0 as *mut MMAPString
    } else {
        (*string).str_0 = tmp
    }
    return string;
}

fn nearest_power(mut base: size_t, mut num: size_t) -> size_t {
    if num > (-1i32 as size_t).wrapping_div(2i32 as libc::size_t) {
        return -1i32 as size_t;
    } else {
        let mut n: size_t = base;
        while n < num {
            n <<= 1i32
        }
        return n;
    };
}

pub unsafe fn mmap_string_sized_new(mut dfl_size: size_t) -> *mut MMAPString {
    let mut string = Box::into_raw(Box::new(MMAPString::default()));

    (*string).allocated_len = 0i32 as size_t;
    (*string).len = 0i32 as size_t;
    (*string).str_0 = 0 as *mut libc::c_char;
    (*string).fd = -1i32;
    (*string).mmapped_size = 0i32 as size_t;
    if mmap_string_maybe_expand(
        string,
        if dfl_size > 2i32 as libc::size_t {
            dfl_size
        } else {
            2i32 as libc::size_t
        },
    )
    .is_null()
    {
        free(string as *mut libc::c_void);
        return 0 as *mut MMAPString;
    }
    *(*string).str_0.offset(0isize) = 0i32 as libc::c_char;
    string
}

pub unsafe fn mmap_string_new_len(
    mut init: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    if len <= 0i32 as libc::size_t {
        return mmap_string_new(b"\x00" as *const u8 as *const libc::c_char);
    }

    let string = mmap_string_sized_new(len);
    if !init.is_null() {
        mmap_string_append_len(string, init, len);
    }
    string
}

pub unsafe fn mmap_string_append_len(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    mmap_string_insert_len(string, (*string).len, val, len)
}

pub unsafe fn mmap_string_free(string: *mut MMAPString) {
    if string.is_null() {
        return;
    }
    let _ = Box::from_raw(string);
}

pub unsafe fn mmap_string_assign(
    mut string: *mut MMAPString,
    mut rval: *const libc::c_char,
) -> *mut MMAPString {
    mmap_string_truncate(string, 0i32 as size_t);
    if mmap_string_append(string, rval).is_null() {
        return std::ptr::null_mut();
    }
    string
}

pub unsafe fn mmap_string_truncate(
    mut string: *mut MMAPString,
    mut len: size_t,
) -> *mut MMAPString {
    (*string).len = if len < (*string).len {
        len
    } else {
        (*string).len
    };
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    string
}

pub unsafe fn mmap_string_set_size(
    mut string: *mut MMAPString,
    mut len: size_t,
) -> *mut MMAPString {
    if len >= (*string).allocated_len {
        if mmap_string_maybe_expand(string, len.wrapping_sub((*string).len)).is_null() {
            return std::ptr::null_mut();
        }
    }
    (*string).len = len;
    *(*string).str_0.offset(len as isize) = 0i32 as libc::c_char;
    string
}

pub unsafe fn mmap_string_append_c(
    mut string: *mut MMAPString,
    mut c: libc::c_char,
) -> *mut MMAPString {
    mmap_string_insert_c(string, (*string).len, c)
}

pub unsafe fn mmap_string_insert_c(
    mut string: *mut MMAPString,
    mut pos: size_t,
    mut c: libc::c_char,
) -> *mut MMAPString {
    if mmap_string_maybe_expand(string, 1i32 as size_t).is_null() {
        return std::ptr::null_mut();
    }

    if pos < (*string).len {
        memmove(
            (*string).str_0.offset(pos as isize).offset(1isize) as *mut libc::c_void,
            (*string).str_0.offset(pos as isize) as *const libc::c_void,
            (*string).len.wrapping_sub(pos),
        );
    }
    *(*string).str_0.offset(pos as isize) = c;
    (*string).len =
        ((*string).len as libc::size_t).wrapping_add(1i32 as libc::size_t) as size_t as size_t;
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    string
}

pub unsafe fn mmap_string_prepend(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
) -> *mut MMAPString {
    mmap_string_insert_len(string, 0i32 as size_t, val, strlen(val))
}

pub unsafe fn mmap_string_prepend_c(
    mut string: *mut MMAPString,
    mut c: libc::c_char,
) -> *mut MMAPString {
    mmap_string_insert_c(string, 0i32 as size_t, c)
}

pub unsafe fn mmap_string_prepend_len(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    mmap_string_insert_len(string, 0i32 as size_t, val, len)
}

pub unsafe fn mmap_string_insert(
    mut string: *mut MMAPString,
    mut pos: size_t,
    mut val: *const libc::c_char,
) -> *mut MMAPString {
    mmap_string_insert_len(string, pos, val, strlen(val))
}

pub unsafe fn mmap_string_erase(
    mut string: *mut MMAPString,
    mut pos: size_t,
    mut len: size_t,
) -> *mut MMAPString {
    if pos.wrapping_add(len) < (*string).len {
        memmove(
            (*string).str_0.offset(pos as isize) as *mut libc::c_void,
            (*string).str_0.offset(pos as isize).offset(len as isize) as *const libc::c_void,
            (*string).len.wrapping_sub(pos.wrapping_add(len)),
        );
    }
    (*string).len = ((*string).len as libc::size_t).wrapping_sub(len) as size_t as size_t;
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    string
}

pub unsafe fn mmap_string_set_ceil(mut ceil: size_t) {
    mmap_string_ceil = ceil;
}

pub unsafe fn mmap_string_ref(string: *mut MMAPString) -> libc::c_int {
    let key = std::ffi::CStr::from_ptr((*string).str_0)
        .to_bytes()
        .to_vec();
    let data = Box::from_raw(string);

    hashtable.lock().unwrap().insert(key, data);

    0
}

pub unsafe fn mmap_string_unref(mut str: *mut libc::c_char) -> libc::c_int {
    if str.is_null() {
        return -1;
    }

    let key = std::ffi::CStr::from_ptr(str).to_bytes().to_vec();

    let string = if let Some(data) = hashtable.lock().unwrap().remove(&key) {
        Box::into_raw(data)
    } else {
        std::ptr::null_mut()
    };

    if !string.is_null() {
        hashtable.lock().unwrap().remove(&key);
        mmap_string_free(string);
        0
    } else {
        -1
    }
}
