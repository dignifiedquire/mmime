use libc;
extern "C" {
    #[no_mangle]
    fn iconv_close(_cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn __error() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn mmap_string_free(string: *mut MMAPString);
    #[no_mangle]
    fn mmap_string_ref(string: *mut MMAPString) -> libc::c_int;
    #[no_mangle]
    fn mmap_string_sized_new(dfl_size: size_t) -> *mut MMAPString;
    #[no_mangle]
    fn mmap_string_set_size(string: *mut MMAPString, len: size_t) -> *mut MMAPString;
    #[no_mangle]
    fn mmap_string_unref(str: *mut libc::c_char) -> libc::c_int;
}
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
 * $Id: charconv.h,v 1.13 2006/06/16 09:25:23 smarinier Exp $
 */
pub type unnamed = libc::c_uint;
pub const MAIL_CHARCONV_ERROR_CONV: unnamed = 3;
pub const MAIL_CHARCONV_ERROR_MEMORY: unnamed = 2;
pub const MAIL_CHARCONV_ERROR_UNKNOWN_CHARSET: unnamed = 1;
pub const MAIL_CHARCONV_NO_ERROR: unnamed = 0;
pub type iconv_t = *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MMAPString {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub allocated_len: size_t,
    pub fd: libc::c_int,
    pub mmapped_size: size_t,
}
/* *
*	define your own conversion.
*		- result is big enough to contain your converted string
*		- result_len contain the maximum size available (out value must contain the final converted size)
*		- your conversion return an error code based on upper enum values
*/
#[no_mangle]
pub static mut extended_charconv: Option<
    unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: size_t,
        _: *mut libc::c_char,
        _: *mut size_t,
    ) -> libc::c_int,
> = None;
#[no_mangle]
pub unsafe extern "C" fn charconv(
    mut tocode: *const libc::c_char,
    mut fromcode: *const libc::c_char,
    mut str: *const libc::c_char,
    mut length: size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut conv: iconv_t = 0 as *mut libc::c_void;
    let mut r: size_t = 0;
    let mut pout: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out_size: size_t = 0;
    let mut old_out_size: size_t = 0;
    let mut count: size_t = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: libc::c_int = 0;
    fromcode = get_valid_charset(fromcode);
    if extended_charconv.is_some() {
        let mut result_length: size_t = 0;
        result_length = length.wrapping_mul(6i32 as libc::c_ulong);
        *result = malloc(
            length
                .wrapping_mul(6i32 as libc::c_ulong)
                .wrapping_add(1i32 as libc::c_ulong),
        ) as *mut libc::c_char;
        if (*result).is_null() {
            res = MAIL_CHARCONV_ERROR_MEMORY as libc::c_int
        } else {
            res = extended_charconv.expect("non-null function pointer")(
                tocode,
                fromcode,
                str,
                length,
                *result,
                &mut result_length,
            );
            if res != MAIL_CHARCONV_NO_ERROR as libc::c_int {
                free(*result as *mut libc::c_void);
            } else {
                out = realloc(
                    *result as *mut libc::c_void,
                    result_length.wrapping_add(1i32 as libc::c_ulong),
                ) as *mut libc::c_char;
                if !out.is_null() {
                    *result = out
                }
                *(*result).offset(result_length as isize) = '\u{0}' as i32 as libc::c_char
            }
        }
        if res != MAIL_CHARCONV_ERROR_UNKNOWN_CHARSET as libc::c_int {
            return res;
        }
    }
    conv = iconv_open(tocode, fromcode);
    if conv == -1i32 as iconv_t {
        res = MAIL_CHARCONV_ERROR_UNKNOWN_CHARSET as libc::c_int
    } else {
        out_size = (6i32 as libc::c_ulong).wrapping_mul(length);
        out = malloc(out_size.wrapping_add(1i32 as libc::c_ulong)) as *mut libc::c_char;
        if out.is_null() {
            res = MAIL_CHARCONV_ERROR_MEMORY as libc::c_int
        } else {
            pout = out;
            old_out_size = out_size;
            r = mail_iconv(
                conv,
                &mut str,
                &mut length,
                &mut pout,
                &mut out_size,
                0 as *mut *mut libc::c_char,
                b"?\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if r == -1i32 as size_t {
                res = MAIL_CHARCONV_ERROR_CONV as libc::c_int;
                free(out as *mut libc::c_void);
            } else {
                iconv_close(conv);
                *pout = '\u{0}' as i32 as libc::c_char;
                count = old_out_size.wrapping_sub(out_size);
                pout = realloc(
                    out as *mut libc::c_void,
                    count.wrapping_add(1i32 as libc::c_ulong),
                ) as *mut libc::c_char;
                if !pout.is_null() {
                    out = pout
                }
                *result = out;
                return MAIL_CHARCONV_NO_ERROR as libc::c_int;
            }
        }
        iconv_close(conv);
    }
    return res;
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
 * $Id: charconv.c,v 1.25 2011/03/29 14:39:55 hoa Exp $
 */
unsafe extern "C" fn mail_iconv(
    mut cd: iconv_t,
    mut inbuf: *mut *const libc::c_char,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut libc::c_char,
    mut outbytesleft: *mut size_t,
    mut inrepls: *mut *mut libc::c_char,
    mut outrepl: *mut libc::c_char,
) -> size_t {
    let mut ret: size_t = 0i32 as size_t;
    let mut ret1: size_t = 0;
    /* XXX - force const to mutable */
    let mut ib: *mut libc::c_char = *inbuf as *mut libc::c_char;
    let mut ibl: size_t = *inbytesleft;
    let mut ob: *mut libc::c_char = *outbuf;
    let mut obl: size_t = *outbytesleft;
    loop {
        ret1 = iconv(cd, &mut ib, &mut ibl, &mut ob, &mut obl);
        if ret1 != -1i32 as size_t {
            ret = (ret as libc::c_ulong).wrapping_add(ret1) as size_t as size_t
        }
        if 0 != ibl && 0 != obl && *__error() == 92i32 {
            if !inrepls.is_null() {
                /* Try replacing the input */
                let mut t: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                t = inrepls;
                while !(*t).is_null() {
                    let mut ib1: *mut libc::c_char = *t;
                    let mut ibl1: size_t = strlen(*t);
                    let mut ob1: *mut libc::c_char = ob;
                    let mut obl1: size_t = obl;
                    iconv(cd, &mut ib1, &mut ibl1, &mut ob1, &mut obl1);
                    if 0 == ibl1 {
                        ib = ib.offset(1isize);
                        ibl = ibl.wrapping_sub(1);
                        ob = ob1;
                        obl = obl1;
                        ret = ret.wrapping_add(1);
                        break;
                    } else {
                        t = t.offset(1isize)
                    }
                }
                if !(*t).is_null() {
                    continue;
                }
            }
            if !outrepl.is_null() {
                /* Try replacing the output */
                let mut n: size_t = strlen(outrepl);
                if n <= obl {
                    memcpy(ob as *mut libc::c_void, outrepl as *const libc::c_void, n);
                    ib = ib.offset(1isize);
                    ibl = ibl.wrapping_sub(1);
                    ob = ob.offset(n as isize);
                    obl = (obl as libc::c_ulong).wrapping_sub(n) as size_t as size_t;
                    ret = ret.wrapping_add(1);
                    continue;
                }
            }
        }
        *inbuf = ib;
        *inbytesleft = ibl;
        *outbuf = ob;
        *outbytesleft = obl;
        return ret;
    }
}
unsafe extern "C" fn get_valid_charset(mut fromcode: *const libc::c_char) -> *const libc::c_char {
    if strcasecmp(fromcode, b"GB2312\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcasecmp(
            fromcode,
            b"GB_2312-80\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
    {
        fromcode = b"GBK\x00" as *const u8 as *const libc::c_char
    } else if strcasecmp(
        fromcode,
        b"iso-8859-8-i\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
        || strcasecmp(
            fromcode,
            b"iso_8859-8-i\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        || strcasecmp(
            fromcode,
            b"iso8859-8-i\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
    {
        fromcode = b"iso-8859-8\x00" as *const u8 as *const libc::c_char
    } else if strcasecmp(
        fromcode,
        b"iso-8859-8-e\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
        || strcasecmp(
            fromcode,
            b"iso_8859-8-e\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        || strcasecmp(
            fromcode,
            b"iso8859-8-e\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
    {
        fromcode = b"iso-8859-8\x00" as *const u8 as *const libc::c_char
    } else if strcasecmp(
        fromcode,
        b"ks_c_5601-1987\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        fromcode = b"euckr\x00" as *const u8 as *const libc::c_char
    } else if strcasecmp(
        fromcode,
        b"iso-2022-jp\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        fromcode = b"iso-2022-jp-2\x00" as *const u8 as *const libc::c_char
    }
    return fromcode;
}
#[no_mangle]
pub unsafe extern "C" fn charconv_buffer(
    mut tocode: *const libc::c_char,
    mut fromcode: *const libc::c_char,
    mut str: *const libc::c_char,
    mut length: size_t,
    mut result: *mut *mut libc::c_char,
    mut result_len: *mut size_t,
) -> libc::c_int {
    let mut conv: iconv_t = 0 as *mut libc::c_void;
    let mut iconv_r: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pout: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out_size: size_t = 0;
    let mut old_out_size: size_t = 0;
    let mut count: size_t = 0;
    let mut res: libc::c_int = 0;
    let mut mmapstr: *mut MMAPString = 0 as *mut MMAPString;
    fromcode = get_valid_charset(fromcode);
    if extended_charconv.is_some() {
        let mut result_length: size_t = 0;
        result_length = length.wrapping_mul(6i32 as libc::c_ulong);
        mmapstr = mmap_string_sized_new(result_length.wrapping_add(1i32 as libc::c_ulong));
        *result_len = 0i32 as size_t;
        if mmapstr.is_null() {
            res = MAIL_CHARCONV_ERROR_MEMORY as libc::c_int
        } else {
            res = extended_charconv.expect("non-null function pointer")(
                tocode,
                fromcode,
                str,
                length,
                (*mmapstr).str_0,
                &mut result_length,
            );
            if res != MAIL_CHARCONV_ERROR_UNKNOWN_CHARSET as libc::c_int {
                if res == MAIL_CHARCONV_NO_ERROR as libc::c_int {
                    *result = (*mmapstr).str_0;
                    res = mmap_string_ref(mmapstr);
                    if res < 0i32 {
                        res = MAIL_CHARCONV_ERROR_MEMORY as libc::c_int;
                        mmap_string_free(mmapstr);
                    } else {
                        mmap_string_set_size(mmapstr, result_length);
                        *result_len = result_length
                    }
                } else {
                    mmap_string_free(mmapstr);
                }
            } else {
                mmap_string_free(mmapstr);
            }
            return res;
        }
    }
    conv = iconv_open(tocode, fromcode);
    if conv == -1i32 as iconv_t {
        res = MAIL_CHARCONV_ERROR_UNKNOWN_CHARSET as libc::c_int
    } else {
        out_size = (6i32 as libc::c_ulong).wrapping_mul(length);
        mmapstr = mmap_string_sized_new(out_size.wrapping_add(1i32 as libc::c_ulong));
        if mmapstr.is_null() {
            res = MAIL_CHARCONV_ERROR_MEMORY as libc::c_int
        } else {
            out = (*mmapstr).str_0;
            pout = out;
            old_out_size = out_size;
            iconv_r = mail_iconv(
                conv,
                &mut str,
                &mut length,
                &mut pout,
                &mut out_size,
                0 as *mut *mut libc::c_char,
                b"?\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if iconv_r == -1i32 as size_t {
                res = MAIL_CHARCONV_ERROR_CONV as libc::c_int
            } else {
                iconv_close(conv);
                *pout = '\u{0}' as i32 as libc::c_char;
                count = old_out_size.wrapping_sub(out_size);
                r = mmap_string_ref(mmapstr);
                if r < 0i32 {
                    res = MAIL_CHARCONV_ERROR_MEMORY as libc::c_int
                } else {
                    *result = out;
                    *result_len = count;
                    return MAIL_CHARCONV_NO_ERROR as libc::c_int;
                }
            }
            mmap_string_free(mmapstr);
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn charconv_buffer_free(mut str: *mut libc::c_char) {
    mmap_string_unref(str);
}
