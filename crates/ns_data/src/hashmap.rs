use ns_error::NsError;
use std::collections::HashMap;

#[repr(C)]
pub struct NsMap {
    // We store the map in a Box so the pointer remains stable for C
    pub internal: *mut HashMap<Vec<u8>, Vec<u8>>,
    pub key_size: usize,
    pub val_size: usize,
    pub len: usize,
}

/// # Safety
///
/// This function creates a new HashMap
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_map_new(dest: *mut NsMap, key_size: usize, val_size: usize) -> NsError {
    if dest.is_null() || key_size == 0 || val_size == 0 {
        return NsError::Any;
    }

    let map = Box::new(HashMap::<Vec<u8>, Vec<u8>>::new());

    unsafe {
        *dest = NsMap {
            internal: Box::into_raw(map),
            key_size,
            val_size,
            len: 0,
        };
    }

    NsError::Success
}

/// # Safety
///
/// This function inserts a new value into HashMap
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_map_insert(
    m: *mut NsMap,
    key_ptr: *const u8,
    val_ptr: *const u8,
) -> NsError {
    if m.is_null() || key_ptr.is_null() || val_ptr.is_null() {
        return NsError::Any;
    }

    let map_info = unsafe { &mut *m };
    let map = unsafe { &mut *map_info.internal };

    // Copy raw bytes from C into Rust-owned Vectors
    let key = unsafe { std::slice::from_raw_parts(key_ptr, map_info.key_size).to_vec() };
    let val = unsafe { std::slice::from_raw_parts(val_ptr, map_info.val_size).to_vec() };

    map.insert(key, val);
    map_info.len = map.len();

    NsError::Success
}

/// # Safety
///
///This function gets a value from the HashMap
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_map_get(
    m: *const NsMap,
    key_ptr: *const u8,
    out_val_ptr: *mut u8,
) -> NsError {
    if m.is_null() || key_ptr.is_null() || out_val_ptr.is_null() {
        return NsError::Any;
    }

    let map_info = unsafe { &*m };
    let map = unsafe { &*map_info.internal };

    let key = unsafe { std::slice::from_raw_parts(key_ptr, map_info.key_size) };

    if let Some(found_val) = map.get(key) {
        unsafe {
            std::ptr::copy_nonoverlapping(found_val.as_ptr(), out_val_ptr, map_info.val_size);
        }
        return NsError::Success;
    }

    NsError::Any // Key not found
}

/// # Safety
///
/// This function frees the HashMap
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_map_free(m: *mut NsMap) {
    if m.is_null() {
        return;
    }

    let map_info = unsafe { &mut *m };
    if !map_info.internal.is_null() {
        // Take ownership back from C and drop it
        unsafe {
            let _ = Box::from_raw(map_info.internal);
        }
    }

    map_info.internal = std::ptr::null_mut();
    map_info.len = 0;
}
