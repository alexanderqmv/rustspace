use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let layout = Layout::from_size_align(4,4).unwrap();
    let ptr = unsafe { alloc(layout) };

    println!("Allocated successfully! {:p}",&ptr);
    if !ptr.is_null() {
        unsafe {
            *(ptr as *mut u32) = 111;
        }
    }

    unsafe { dealloc(ptr, layout) };
    println!("Deallocated successfully! {:p}",&ptr);
}
