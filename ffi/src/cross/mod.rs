use libc::{c_void, size_t, ssize_t};
use std::mem;

extern "C" {
    pub fn malloc(sz: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
}

// given an N generate a matrix filled in clockwise
#[no_mangle]
pub extern "C" fn matrix(N: size_t) -> *mut *mut size_t {
    unsafe {
        let start: *mut *mut size_t =
            malloc(N * mem::size_of::<*const size_t>()) as *mut *mut size_t;

        for i in 0..N {
            *start.offset(i as isize) = malloc(N * mem::size_of::<size_t>()) as *mut size_t;
        }

        let mut T = 0;
        let mut B = N;
        let mut L = 0;
        let mut R = N;

        let mut c = 1;
        loop {
            if T == N {
                break;
            }

            for i in L..R {
                (*(*start.offset(T as isize)).offset(i as isize)) = c;
                c = c + 1;
            }
            T = T + 1;

            for i in T..B {
                (*(*start.offset(i as isize)).offset((R - 1) as isize)) = c;
                c = c + 1;
            }
            R = R - 1;

            for i in (L..R).rev() {
                (*(*start.offset((B - 1) as isize)).offset(i as isize)) = c;
                c = c + 1;
            }
            B = B - 1;

            for i in (T..B).rev() {
                (*(*start.offset(i as isize)).offset(L as isize)) = c;
                c = c + 1;
            }
            L = L + 1;
        }

        start
    }
}

#[no_mangle]
pub extern "C" fn free_matrix(m: *mut *mut size_t, sz: size_t) {
    for i in 0..sz {
        unsafe {
            free(*m.offset(i as isize) as *mut c_void);
        }
    }

    unsafe {
        free(m as *mut c_void);
    }
}
