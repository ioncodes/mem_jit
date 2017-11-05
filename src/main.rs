extern crate libc;
extern crate aligned_alloc;
extern crate region;

mod jit_memory;

use std::mem;
use region::{View, Protection};
use std::ptr::{read_volatile, write_volatile};
use jit_memory::JitMemory;

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

fn write(contents: *mut u8, value: u8) {
    unsafe {
        write_volatile(contents, value);
    }
}

fn read(contents: *mut u8) -> u8 {
    unsafe { read_volatile(contents) }
}

fn main() {
    let (contents, ptr) = create_mem();
    let jit = JitMemory::new(1);
    write(contents, 12);
    let val = read(contents);
    println!("{:?}", val);
    free_mem(ptr);
}
