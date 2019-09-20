use libc;

#[derive(Clone, Default)]
pub struct clist(pub Vec<*mut libc::c_void>);

impl clist {
    pub fn last(&self) -> *mut libc::c_void {
        match self.0.last() {
            Some(v) => *v,
            None => std::ptr::null_mut(),
        }
    }

    pub fn first(&self) -> *mut libc::c_void {
        match self.0.first() {
            Some(v) => *v,
            None => std::ptr::null_mut(),
        }
    }
}

impl std::ops::Deref for clist {
    type Target = [*mut libc::c_void];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type clist_func =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> ()>;

/* Allocate a new pointer list */
pub fn clist_new() -> *mut clist {
    Box::into_raw(Box::new(Default::default()))
}
/* Destroys a list. Data pointed by data pointers is NOT freed. */
pub unsafe fn clist_free(mut lst: *mut clist) {
    Box::from_raw(lst);
}

pub unsafe fn clist_insert_end(mut lst: *mut clist, data: *mut libc::c_void) {
    (*lst).0.push(data);
}

/// Deletes the element pointed by the index.
pub unsafe fn clist_delete(mut lst: *mut clist, i: usize) {
    (*lst).0.remove(i);
}

pub unsafe fn clist_foreach(
    mut lst: *mut clist,
    mut func: clist_func,
    mut data: *mut libc::c_void,
) {
    for el in &(*lst).0 {
        func.expect("non-null function pointer")(*el, data);
    }
}

pub unsafe fn clist_nth_data(lst: *mut clist, i: usize) -> *mut libc::c_void {
    (*lst).0[i]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clist_iterator() {
        unsafe {
            let mut c = clist_new();
            assert!(!c.is_null());

            let a = 0u32;
            clist_insert_end(c, a as _);

            assert_eq!((*c).len(), 1);

            /* Only one iteration */
            for data in (*c).iter() {
                assert_eq!(*data as u32, a);
            }
            assert_eq!((*c).len(), 1);

            clist_free(c);
        }
    }
}
