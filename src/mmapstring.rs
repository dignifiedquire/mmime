use libc;
extern "C" {
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    /* Adds an entry in the hash table.
    Length can be 0 if key/value are strings.
    If an entry already exists for this key, it is replaced, and its value
    is returned. Otherwise, the data pointer will be NULL and the length
    field be set to TRUE or FALSe to indicate success or failure. */
    #[no_mangle]
    fn chash_set(
        hash: *mut chash,
        key: *mut chashdatum,
        value: *mut chashdatum,
        oldvalue: *mut chashdatum,
    ) -> libc::c_int;
    /* Allocates a new (empty) hash using this initial size and the given flags,
      specifying which data should be copied in the hash.
       CHASH_COPYNONE  : Keys/Values are not copied.
       CHASH_COPYKEY   : Keys are dupped and freed as needed in the hash.
       CHASH_COPYVALUE : Values are dupped and freed as needed in the hash.
       CHASH_COPYALL   : Both keys and values are dupped in the hash.
    */
    #[no_mangle]
    fn chash_new(size: libc::c_uint, flags: libc::c_int) -> *mut chash;
    #[no_mangle]
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
    /* Frees a hash */
    #[no_mangle]
    fn chash_free(hash: *mut chash);
    /* Removes the entry associated to this key if it is found in the hash table,
    and returns its contents if not dupped (otherwise, pointer will be NULL
    and len TRUE). If entry is not found both pointer and len will be NULL. */
    #[no_mangle]
    fn chash_delete(
        hash: *mut chash,
        key: *mut chashdatum,
        oldvalue: *mut chashdatum,
    ) -> libc::c_int;
    /* Retrieves the data associated to the key if it is found in the hash table.
    The data pointer and the length will be NULL if not found*/
    #[no_mangle]
    fn chash_get(hash: *mut chash, key: *mut chashdatum, result: *mut chashdatum) -> libc::c_int;
}
pub type __darwin_size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type size_t = __darwin_size_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MMAPString {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub allocated_len: size_t,
    pub fd: libc::c_int,
    pub mmapped_size: size_t,
}
/*
 * libEtPan! -- a mail stuff library
 *
 * Copyright (C) 2001, 2005 - DINH Viet Hoa
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the libEtPan! project nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * $Id: mmapstring.h,v 1.14 2008/02/28 14:06:27 colinleroy Exp $
 */
/*
#define TMPDIR "/tmp"
*/
pub type MMAPString = _MMAPString;
/*
 * libEtPan! -- a mail stuff library
 *
 * chash - Implements generic hash tables.
 *
 * Copyright (c) 1999-2005, Ga�l Roualland <gael.roualland@iname.com>
 * interface changes - 2005 - DINH Viet Hoa
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the libEtPan! project nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * $Id: chash.h,v 1.16 2010/11/16 20:46:35 hoa Exp $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chashdatum {
    pub data: *mut libc::c_void,
    pub len: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chash {
    pub size: libc::c_uint,
    pub count: libc::c_uint,
    pub copyvalue: libc::c_int,
    pub copykey: libc::c_int,
    pub cells: *mut *mut chashcell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chashcell {
    pub func: libc::c_uint,
    pub key: chashdatum,
    pub value: chashdatum,
    pub next: *mut chashcell,
}
/*
char * old_non_mmapped_str;
*/
/* configure location of mmaped files */
#[no_mangle]
pub unsafe extern "C" fn mmap_string_set_tmpdir(mut directory: *const libc::c_char) {
    strncpy(tmpdir.as_mut_ptr(), directory, 1024i32 as libc::c_ulong);
    tmpdir[(1024i32 - 1i32) as usize] = 0i32 as libc::c_char;
}
/*
 * libEtPan! -- a mail stuff library
 *
 * Copyright (C) 2001, 2005 - DINH Viet Hoa
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the libEtPan! project nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * $Id: mmapstring.c,v 1.26 2011/03/14 22:49:16 hoa Exp $
 */
static mut tmpdir: [libc::c_char; 1024] = [
    47, 116, 109, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];
/* Strings
 */
#[no_mangle]
pub unsafe extern "C" fn mmap_string_new(mut init: *const libc::c_char) -> *mut MMAPString {
    let mut string: *mut MMAPString = 0 as *mut MMAPString;
    string = mmap_string_sized_new(if !init.is_null() {
        strlen(init).wrapping_add(2i32 as libc::c_ulong)
    } else {
        2i32 as libc::c_ulong
    });
    if string.is_null() {
        return 0 as *mut MMAPString;
    }
    if !init.is_null() {
        mmap_string_append(string, init);
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_append(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
) -> *mut MMAPString {
    return mmap_string_insert_len(string, (*string).len, val, strlen(val));
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_insert_len(
    mut string: *mut MMAPString,
    mut pos: size_t,
    mut val: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    if mmap_string_maybe_expand(string, len).is_null() {
        return 0 as *mut MMAPString;
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
    (*string).len = ((*string).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    return string;
}
unsafe extern "C" fn mmap_string_maybe_expand(
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
                .wrapping_add(1i32 as libc::c_ulong),
        );
        newstring = mmap_string_realloc_memory(string);
        if newstring.is_null() {
            (*string).allocated_len = old_size
        }
        return newstring;
    }
    return string;
}
/* Strings.
 */
/* SEB */
unsafe extern "C" fn mmap_string_realloc_memory(mut string: *mut MMAPString) -> *mut MMAPString {
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
/* MMAPString */
#[inline]
unsafe extern "C" fn nearest_power(mut base: size_t, mut num: size_t) -> size_t {
    if num > (-1i32 as size_t).wrapping_div(2i32 as libc::c_ulong) {
        return -1i32 as size_t;
    } else {
        let mut n: size_t = base;
        while n < num {
            n <<= 1i32
        }
        return n;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_sized_new(mut dfl_size: size_t) -> *mut MMAPString {
    let mut string: *mut MMAPString = 0 as *mut MMAPString;
    string = malloc(::std::mem::size_of::<MMAPString>() as libc::c_ulong) as *mut MMAPString;
    if string.is_null() {
        return 0 as *mut MMAPString;
    }
    (*string).allocated_len = 0i32 as size_t;
    (*string).len = 0i32 as size_t;
    (*string).str_0 = 0 as *mut libc::c_char;
    (*string).fd = -1i32;
    (*string).mmapped_size = 0i32 as size_t;
    if mmap_string_maybe_expand(
        string,
        if dfl_size > 2i32 as libc::c_ulong {
            dfl_size
        } else {
            2i32 as libc::c_ulong
        },
    )
    .is_null()
    {
        free(string as *mut libc::c_void);
        return 0 as *mut MMAPString;
    }
    *(*string).str_0.offset(0isize) = 0i32 as libc::c_char;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_new_len(
    mut init: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    let mut string: *mut MMAPString = 0 as *mut MMAPString;
    if len <= 0i32 as libc::c_ulong {
        return mmap_string_new(b"\x00" as *const u8 as *const libc::c_char);
    } else {
        string = mmap_string_sized_new(len);
        if string.is_null() {
            return string;
        }
        if !init.is_null() {
            mmap_string_append_len(string, init, len);
        }
        return string;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_append_len(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    return mmap_string_insert_len(string, (*string).len, val, len);
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_free(mut string: *mut MMAPString) {
    if string.is_null() {
        return;
    }
    free((*string).str_0 as *mut libc::c_void);
    free(string as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_assign(
    mut string: *mut MMAPString,
    mut rval: *const libc::c_char,
) -> *mut MMAPString {
    mmap_string_truncate(string, 0i32 as size_t);
    if mmap_string_append(string, rval).is_null() {
        return 0 as *mut MMAPString;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_truncate(
    mut string: *mut MMAPString,
    mut len: size_t,
) -> *mut MMAPString {
    (*string).len = if len < (*string).len {
        len
    } else {
        (*string).len
    };
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_set_size(
    mut string: *mut MMAPString,
    mut len: size_t,
) -> *mut MMAPString {
    if len >= (*string).allocated_len {
        if mmap_string_maybe_expand(string, len.wrapping_sub((*string).len)).is_null() {
            return 0 as *mut MMAPString;
        }
    }
    (*string).len = len;
    *(*string).str_0.offset(len as isize) = 0i32 as libc::c_char;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_append_c(
    mut string: *mut MMAPString,
    mut c: libc::c_char,
) -> *mut MMAPString {
    return mmap_string_insert_c(string, (*string).len, c);
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_insert_c(
    mut string: *mut MMAPString,
    mut pos: size_t,
    mut c: libc::c_char,
) -> *mut MMAPString {
    if mmap_string_maybe_expand(string, 1i32 as size_t).is_null() {
        return 0 as *mut MMAPString;
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
        ((*string).len as libc::c_ulong).wrapping_add(1i32 as libc::c_ulong) as size_t as size_t;
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_prepend(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
) -> *mut MMAPString {
    return mmap_string_insert_len(string, 0i32 as size_t, val, strlen(val));
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_prepend_c(
    mut string: *mut MMAPString,
    mut c: libc::c_char,
) -> *mut MMAPString {
    return mmap_string_insert_c(string, 0i32 as size_t, c);
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_prepend_len(
    mut string: *mut MMAPString,
    mut val: *const libc::c_char,
    mut len: size_t,
) -> *mut MMAPString {
    return mmap_string_insert_len(string, 0i32 as size_t, val, len);
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_insert(
    mut string: *mut MMAPString,
    mut pos: size_t,
    mut val: *const libc::c_char,
) -> *mut MMAPString {
    return mmap_string_insert_len(string, pos, val, strlen(val));
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_erase(
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
    (*string).len = ((*string).len as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
    *(*string).str_0.offset((*string).len as isize) = 0i32 as libc::c_char;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_set_ceil(mut ceil: size_t) {
    mmap_string_ceil = ceil;
}
static mut mmap_string_ceil: size_t = (8i32 * 1024i32 * 1024i32) as size_t;
#[no_mangle]
pub unsafe extern "C" fn mmap_string_ref(mut string: *mut MMAPString) -> libc::c_int {
    let mut ht: *mut chash = 0 as *mut chash;
    let mut r: libc::c_int = 0;
    let mut key: chashdatum = chashdatum {
        data: 0 as *mut libc::c_void,
        len: 0,
    };
    let mut data: chashdatum = chashdatum {
        data: 0 as *mut libc::c_void,
        len: 0,
    };
    pthread_mutex_lock(&mut mmapstring_lock);
    if mmapstring_hashtable.is_null() {
        mmapstring_hashtable_init();
    }
    ht = mmapstring_hashtable;
    if ht.is_null() {
        pthread_mutex_unlock(&mut mmapstring_lock);
        return -1i32;
    }
    key.data = &mut (*string).str_0 as *mut *mut libc::c_char as *mut libc::c_void;
    key.len = ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as libc::c_uint;
    data.data = string as *mut libc::c_void;
    data.len = 0i32 as libc::c_uint;
    r = chash_set(
        mmapstring_hashtable,
        &mut key,
        &mut data,
        0 as *mut chashdatum,
    );
    pthread_mutex_unlock(&mut mmapstring_lock);
    if r < 0i32 {
        return r;
    }
    return 0i32;
}
/* MMAPString references */
static mut mmapstring_lock: pthread_mutex_t = _opaque_pthread_mutex_t {
    __sig: 0x32aaaba7i32 as libc::c_long,
    __opaque: [
        0i32 as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
};
static mut mmapstring_hashtable: *mut chash = 0 as *const chash as *mut chash;
unsafe extern "C" fn mmapstring_hashtable_init() {
    mmapstring_hashtable = chash_new(13i32 as libc::c_uint, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mmap_string_unref(mut str: *mut libc::c_char) -> libc::c_int {
    let mut string: *mut MMAPString = 0 as *mut MMAPString;
    let mut ht: *mut chash = 0 as *mut chash;
    let mut key: chashdatum = chashdatum {
        data: 0 as *mut libc::c_void,
        len: 0,
    };
    let mut data: chashdatum = chashdatum {
        data: 0 as *mut libc::c_void,
        len: 0,
    };
    let mut r: libc::c_int = 0;
    if str.is_null() {
        return -1i32;
    }
    pthread_mutex_lock(&mut mmapstring_lock);
    ht = mmapstring_hashtable;
    if ht.is_null() {
        pthread_mutex_unlock(&mut mmapstring_lock);
        return -1i32;
    }
    key.data = &mut str as *mut *mut libc::c_char as *mut libc::c_void;
    key.len = ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as libc::c_uint;
    r = chash_get(ht, &mut key, &mut data);
    if r < 0i32 {
        string = 0 as *mut MMAPString
    } else {
        string = data.data as *mut MMAPString
    }
    if !string.is_null() {
        chash_delete(ht, &mut key, 0 as *mut chashdatum);
        if chash_count(ht) == 0i32 as libc::c_uint {
            chash_free(ht);
            mmapstring_hashtable = 0 as *mut chash
        }
    }
    pthread_mutex_unlock(&mut mmapstring_lock);
    if !string.is_null() {
        mmap_string_free(string);
        return 0i32;
    } else {
        return -1i32;
    };
}
#[inline]
unsafe extern "C" fn chash_count(mut hash: *mut chash) -> libc::c_uint {
    return (*hash).count;
}
/*
 * libEtPan! -- a mail stuff library
 *
 * Copyright (C) 2001, 2014 - DINH Viet Hoa
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the libEtPan! project nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
#[no_mangle]
pub unsafe extern "C" fn mmapstring_init_lock() {}
#[no_mangle]
pub unsafe extern "C" fn mmapstring_uninit_lock() {}
