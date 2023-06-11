use std::alloc::{alloc, dealloc, Layout};
use std::mem::align_of;
use std::ptr;



#[derive(Debug)]
struct Ref<'a,T> {
    x: &'a T,
    y: &'a T,
    z: &'a T,
}


fn main() {
    let (x,y,z) = (1,2,3);

    let r = Ref {
        x: &x,
        y: &y,
        z: &z,
    };

    println!("{r:?}");
}

