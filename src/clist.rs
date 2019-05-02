use libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
}
/*
 * libEtPan! -- a mail stuff library
 *
 * clist - Implements simple generic double-linked pointer lists
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
 * $Id: clist.h,v 1.13 2011/05/09 21:49:46 hoa Exp $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clistcell_s {
    pub data: *mut libc::c_void,
    pub previous: *mut clistcell_s,
    pub next: *mut clistcell_s,
}
pub type clistcell = clistcell_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clist_s {
    pub first: *mut clistcell,
    pub last: *mut clistcell,
    pub count: libc::c_int,
}
pub type clist = clist_s;
pub type clistiter = clistcell;
pub type clist_func =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> ()>;
/* Allocate a new pointer list */
#[no_mangle]
pub unsafe extern "C" fn clist_new() -> *mut clist {
    let mut lst: *mut clist = 0 as *mut clist;
    lst = malloc(::std::mem::size_of::<clist>() as libc::c_ulong) as *mut clist;
    if lst.is_null() {
        return 0 as *mut clist;
    }
    (*lst).last = 0 as *mut clistcell;
    (*lst).first = (*lst).last;
    (*lst).count = 0i32;
    return lst;
}
/* Destroys a list. Data pointed by data pointers is NOT freed. */
#[no_mangle]
pub unsafe extern "C" fn clist_free(mut lst: *mut clist) {
    let mut l1: *mut clistcell = 0 as *mut clistcell;
    let mut l2: *mut clistcell = 0 as *mut clistcell;
    l1 = (*lst).first;
    while !l1.is_null() {
        l2 = (*l1).next;
        free(l1 as *mut libc::c_void);
        l1 = l2
    }
    free(lst as *mut libc::c_void);
}
/* Some of the following routines can be implemented as macros to
be faster. If you don't want it, define NO_MACROS */
/* Inserts this data pointer before the element pointed by the iterator */
#[no_mangle]
pub unsafe extern "C" fn clist_insert_before(
    mut lst: *mut clist,
    mut iter: *mut clistiter,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut c: *mut clistcell = 0 as *mut clistcell;
    c = malloc(::std::mem::size_of::<clistcell>() as libc::c_ulong) as *mut clistcell;
    if c.is_null() {
        return -1i32;
    }
    (*c).data = data;
    (*lst).count += 1;
    if (*lst).first == (*lst).last && (*lst).last.is_null() {
        (*c).next = 0 as *mut clistcell_s;
        (*c).previous = (*c).next;
        (*lst).last = c;
        (*lst).first = (*lst).last;
        return 0i32;
    }
    if iter.is_null() {
        (*c).previous = (*lst).last;
        (*(*c).previous).next = c;
        (*c).next = 0 as *mut clistcell_s;
        (*lst).last = c;
        return 0i32;
    }
    (*c).previous = (*iter).previous;
    (*c).next = iter;
    (*(*c).next).previous = c;
    if !(*c).previous.is_null() {
        (*(*c).previous).next = c
    } else {
        (*lst).first = c
    }
    return 0i32;
}
/* Inserts this data pointer after the element pointed by the iterator */
#[no_mangle]
pub unsafe extern "C" fn clist_insert_after(
    mut lst: *mut clist,
    mut iter: *mut clistiter,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut c: *mut clistcell = 0 as *mut clistcell;
    c = malloc(::std::mem::size_of::<clistcell>() as libc::c_ulong) as *mut clistcell;
    if c.is_null() {
        return -1i32;
    }
    (*c).data = data;
    (*lst).count += 1;
    if (*lst).first == (*lst).last && (*lst).last.is_null() {
        (*c).next = 0 as *mut clistcell_s;
        (*c).previous = (*c).next;
        (*lst).last = c;
        (*lst).first = (*lst).last;
        return 0i32;
    }
    if iter.is_null() {
        (*c).previous = (*lst).last;
        (*(*c).previous).next = c;
        (*c).next = 0 as *mut clistcell_s;
        (*lst).last = c;
        return 0i32;
    }
    (*c).previous = iter;
    (*c).next = (*iter).next;
    if !(*c).next.is_null() {
        (*(*c).next).previous = c
    } else {
        (*lst).last = c
    }
    (*(*c).previous).next = c;
    return 0i32;
}
/* Deletes the element pointed by the iterator.
Returns an iterator to the next element. */
#[no_mangle]
pub unsafe extern "C" fn clist_delete(
    mut lst: *mut clist,
    mut iter: *mut clistiter,
) -> *mut clistiter {
    let mut ret: *mut clistiter = 0 as *mut clistiter;
    if iter.is_null() {
        return 0 as *mut clistiter;
    }
    if !(*iter).previous.is_null() {
        (*(*iter).previous).next = (*iter).next
    } else {
        (*lst).first = (*iter).next
    }
    if !(*iter).next.is_null() {
        (*(*iter).next).previous = (*iter).previous;
        ret = (*iter).next
    } else {
        (*lst).last = (*iter).previous;
        ret = 0 as *mut clistiter
    }
    free(iter as *mut libc::c_void);
    (*lst).count -= 1;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn clist_foreach(
    mut lst: *mut clist,
    mut func: clist_func,
    mut data: *mut libc::c_void,
) {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    cur = (*lst).first;
    while !cur.is_null() {
        func.expect("non-null function pointer")((*cur).data, data);
        cur = (*cur).next
    }
}
#[no_mangle]
pub unsafe extern "C" fn clist_concat(mut dest: *mut clist, mut src: *mut clist) {
    if !(*src).first.is_null() {
        if (*dest).last.is_null() {
            (*dest).first = (*src).first;
            (*dest).last = (*src).last
        } else {
            (*(*dest).last).next = (*src).first;
            (*(*src).first).previous = (*dest).last;
            (*dest).last = (*src).last
        }
    }
    (*dest).count += (*src).count;
    (*src).first = 0 as *mut clistcell;
    (*src).last = (*src).first;
}
#[no_mangle]
pub unsafe extern "C" fn clist_nth_data(
    mut lst: *mut clist,
    mut indx: libc::c_int,
) -> *mut libc::c_void {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    cur = internal_clist_nth(lst, indx);
    if cur.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*cur).data;
}
#[inline]
unsafe extern "C" fn internal_clist_nth(
    mut lst: *mut clist,
    mut indx: libc::c_int,
) -> *mut clistiter {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    cur = (*lst).first;
    while indx > 0i32 && !cur.is_null() {
        cur = (*cur).next;
        indx -= 1
    }
    if cur.is_null() {
        return 0 as *mut clistiter;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn clist_nth(mut lst: *mut clist, mut indx: libc::c_int) -> *mut clistiter {
    return internal_clist_nth(lst, indx);
}
