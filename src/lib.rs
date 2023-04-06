use std::os::raw::{c_uchar, c_uint};

use rand::RngCore;

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn get_random() -> u32 {
    rand::random::<u32>()
}

#[repr(C)]
pub struct Buffer {
    pub ptr: *const c_uchar,
    pub size: c_uint,
}

#[no_mangle]
pub extern "C" fn get_random_number(size: u32) -> Buffer {
    let bytes: Box<[u8]> = (0..size).map(|_| rand::random::<u8>()).collect();

    let ptr = Box::into_raw(bytes) as *const c_uchar;

    Buffer {
        ptr,
        size: size as c_uint,
    }
}

#[no_mangle]
pub extern "C" fn free_buffer(buffer: Buffer) {
    unsafe {
        let _ = Box::from_raw(buffer.ptr as *mut u8);
    }
}
