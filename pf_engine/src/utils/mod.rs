pub mod raw_mesh;
use std::hash::Hasher;

/// Performs hashing of a sized value by interpreting it as raw memory.
pub fn hash_as_bytes<T: Sized, H: Hasher>(value: &T, hasher: &mut H) {
    hasher.write(value_as_u8_slice(value))
}

/// "Transmutes" value of any sized type to a slice of bytes.
pub fn value_as_u8_slice<T: Sized>(v: &T) -> &'_ [u8] {
    // SAFETY: It is safe to reinterpret data to read it.
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}


/// "Transmutes" array of any sized type to a slice of bytes.
pub fn array_as_u8_slice<T: Sized>(v: &[T]) -> &'_ [u8] {
    // SAFETY: It is safe to reinterpret data to read it.
    unsafe {
        std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of::<T>() * v.len())
    }
}
