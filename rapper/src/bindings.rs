/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Buffer {
    pub ptr: *const ::std::os::raw::c_uchar,
    pub size: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_Buffer() {
    const UNINIT: ::std::mem::MaybeUninit<Buffer> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Buffer>(),
        16usize,
        concat!("Size of: ", stringify!(Buffer))
    );
    assert_eq!(
        ::std::mem::align_of::<Buffer>(),
        8usize,
        concat!("Alignment of ", stringify!(Buffer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Buffer),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Buffer),
            "::",
            stringify!(size)
        )
    );
}
extern "C" {
    pub fn add(a: i32, b: i32) -> i32;
}
extern "C" {
    pub fn get_random() -> u32;
}
extern "C" {
    pub fn get_random_number(size: u32) -> Buffer;
}
extern "C" {
    pub fn free_buffer(buffer: Buffer);
}
