use std::mem;

pub fn as_byte_array<T>(data: &T) -> &[u8] {
    let ptr = data as *const T as *const u8;
    unsafe {
        std::slice::from_raw_parts(ptr, mem::size_of::<T>())
    }
}