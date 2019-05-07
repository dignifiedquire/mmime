use libc;

use crate::x::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct carray {
    pub array: *mut *mut libc::c_void,
    pub len: libc::c_uint,
    pub max: libc::c_uint,
}

pub unsafe fn carray_count(array: *mut carray) -> libc::c_uint {
    (*array).len
}

/* Creates a new array of pointers, with initsize preallocated cells */
pub unsafe fn carray_new(mut initsize: libc::c_uint) -> *mut carray {
    let mut array: *mut carray = 0 as *mut carray;
    array = malloc(::std::mem::size_of::<carray>() as libc::c_ulong) as *mut carray;
    if array.is_null() {
        return 0 as *mut carray;
    }
    if initsize < 4i32 as libc::c_uint {
        initsize = 4i32 as libc::c_uint
    }
    (*array).len = 0i32 as libc::c_uint;
    (*array).max = initsize;
    (*array).array = malloc(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(initsize as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    if (*array).array.is_null() {
        free(array as *mut libc::c_void);
        return 0 as *mut carray;
    }
    return array;
}
/* Adds the pointer to data in the array.
Returns the index of the pointer in the array or -1 on error */
pub unsafe fn carray_add(
    mut array: *mut carray,
    mut data: *mut libc::c_void,
    mut indx: *mut libc::c_uint,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = carray_set_size(array, (*array).len.wrapping_add(1i32 as libc::c_uint));
    if r < 0i32 {
        return r;
    }
    let ref mut fresh0 = *(*array)
        .array
        .offset((*array).len.wrapping_sub(1i32 as libc::c_uint) as isize);
    *fresh0 = data;
    if !indx.is_null() {
        *indx = (*array).len.wrapping_sub(1i32 as libc::c_uint)
    }
    return 0i32;
}

pub unsafe fn carray_set_size(mut array: *mut carray, mut new_size: libc::c_uint) -> libc::c_int {
    if new_size > (*array).max {
        let mut n: libc::c_uint = (*array).max.wrapping_mul(2i32 as libc::c_uint);
        let mut new: *mut libc::c_void = 0 as *mut libc::c_void;
        while n <= new_size {
            n = n.wrapping_mul(2i32 as libc::c_uint)
        }
        new = realloc(
            (*array).array as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut *mut libc::c_void as *mut libc::c_void;
        if new.is_null() {
            return -1i32;
        }
        (*array).array = new as *mut *mut libc::c_void;
        (*array).max = n
    }
    (*array).len = new_size;
    return 0i32;
}
/* Removes the cell at this index position. Returns TRUE on success.
Order of elements in the array IS changed. */
pub unsafe fn carray_delete(mut array: *mut carray, mut indx: libc::c_uint) -> libc::c_int {
    if indx >= (*array).len {
        return -1i32;
    }
    (*array).len = (*array).len.wrapping_sub(1);
    if indx != (*array).len {
        let ref mut fresh1 = *(*array).array.offset(indx as isize);
        *fresh1 = *(*array).array.offset((*array).len as isize)
    }
    return 0i32;
}
/* Removes the cell at this index position. Returns TRUE on success.
Order of elements in the array IS not changed. */
pub unsafe fn carray_delete_slow(mut array: *mut carray, mut indx: libc::c_uint) -> libc::c_int {
    if indx >= (*array).len {
        return -1i32;
    }
    (*array).len = (*array).len.wrapping_sub(1);
    if indx != (*array).len {
        memmove(
            (*array).array.offset(indx as isize) as *mut libc::c_void,
            (*array).array.offset(indx as isize).offset(1isize) as *const libc::c_void,
            ((*array).len.wrapping_sub(indx) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
        );
    }
    return 0i32;
}
/* remove without decreasing the size of the array */
pub unsafe fn carray_delete_fast(mut array: *mut carray, mut indx: libc::c_uint) -> libc::c_int {
    if indx >= (*array).len {
        return -1i32;
    }
    let ref mut fresh2 = *(*array).array.offset(indx as isize);
    *fresh2 = 0 as *mut libc::c_void;
    return 0i32;
}

pub unsafe fn carray_free(mut array: *mut carray) {
    free((*array).array as *mut libc::c_void);
    free(array as *mut libc::c_void);
}
