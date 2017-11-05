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

fn write(contents: *mut u8, offset: isize, value: u8) {
    unsafe {
        write_volatile(contents.offset(offset), value);
    }
}

fn read(contents: *mut u8, offset: isize) -> u8 {
    unsafe { read_volatile(contents.offset(offset)) }
}

fn main() {
    let (contents, ptr) = create_mem();
    let mut jit = JitMemory::new(1);
    write(contents, 0, 12);
    let val = read(contents, 0);
    jit.emit_bytes(vec![0x48, 0xb8]);
    jit.emit64(ptr as u64); // mov rax, <64 bit address>
    let rax = jit.execute();
    assert_eq!(ptr as u64, rax as u64);
    jit.emit_bytes(vec![0x48, 0x83, 0x00, 0x05]); // add qword ptr [rax+0], 5
    let _ = jit.execute();
    let m = read(contents, 0);
    println!("{:?}", m);
    free_mem(ptr);
}
