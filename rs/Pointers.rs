use std::alloc::{alloc, dealloc, realloc, Layout};
use std::mem::size_of_val;
use std::ptr::{null, null_mut};

fn main() {
    let layout = Layout::from_size_align(4, 4).unwrap();
    let ptr: *mut u32 = unsafe { alloc(layout) as *mut u32 };
    if !ptr.is_null() {
        unsafe {
            *ptr = 1234;
        }
    }

    let ptr2: *mut u32 = unsafe { alloc(layout) as *mut u32 };
    if !ptr2.is_null() {
        unsafe {
            *ptr2 = 1234;
        }
    }

    let ptr3: *mut u32 = unsafe { alloc(layout) as *mut u32 };

    if !ptr3.is_null() {
        unsafe {
            *ptr3 = *ptr * *ptr2;
        }
    }

    println!("{}", unsafe { *ptr3 });

    unsafe {
        dealloc(ptr as *mut u8, layout);
        dealloc(ptr2 as *mut u8, layout);
        dealloc(ptr3 as *mut u8, layout);
    }
    println!("Successfully deallocated!");
}
