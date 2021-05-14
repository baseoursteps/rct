extern crate heap;
extern crate libc;
extern crate rand;

use heap::utils;
use rand::Rng;
use std::time::Instant;

use libc::{c_void, size_t};

extern "C" {
    fn qsort(
        base: *mut c_void,
        msize: size_t,
        sz: size_t,
        cmp: extern "C" fn(a: *const c_void, b: *const c_void) -> i32,
    );
}

extern "C" fn cmp(a: *const c_void, b: *const c_void) -> i32 {
    let aa = a as *const i32;
    let bb = b as *const i32;

    unsafe {
        match (*aa).cmp(&*bb) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..10_000_000 {
        nums.push(rng.gen());
    }

    let mut nums1 = nums.clone();

    let now = Instant::now();
    utils::heap_sort(&mut nums, utils::Type::Min);
    println!("Rust heapsort took {} millis", now.elapsed().as_millis());

    unsafe {
        let now1 = Instant::now();
        qsort(nums1.as_mut_ptr() as *mut c_void, nums.len(), 4, cmp);
        println!("qsort took {} millis", now1.elapsed().as_millis());
    }
}
