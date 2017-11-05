extern crate libc;
extern crate aligned_alloc;
extern crate region;

use std::mem;
use region::{View, Protection};
use std::ptr::{read_volatile, write_volatile};

extern "C" {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

fn create_mem() -> (*mut u8, *mut ()) {
    let contents: *mut u8;
    let ptr: *mut ();
    let size = 1 * 4096;
    unsafe {
        ptr = aligned_alloc::aligned_alloc(4096, size);
        let mut view = View::new(ptr as *const u8, size).unwrap();
        view.set_prot(Protection::ReadWrite.into()).unwrap();

        memset(ptr as *mut libc::c_void, 0x00, size);

        contents = mem::transmute(ptr);
    }
    (contents, ptr)
}

fn free_mem(ptr: *mut ()) {
    unsafe {
        aligned_alloc::aligned_free(ptr);
    }
}

fn main() {
    let (contents, ptr) = create_mem();
    unsafe {
        println!("Offset: {:?}", &*contents.offset(0));
        println!("From Pointer: {:?}", read_volatile(contents));
        write_volatile(contents, 12); // add [contents+0], 12
        println!("Offset: {:?}", &*contents.offset(0));
        println!("From Pointer: {:?}", read_volatile(contents));
    }
    free_mem(ptr);
}
