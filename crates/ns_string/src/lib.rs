use ns_error::NsError;
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::os::raw::c_char;

// Heap allocation
#[repr(C)]
pub struct NsStringHeap {
    pub ptr: *mut u8,
    pub capacity: usize,
}

// Union: 24 bytes of inline chars or the heap struct
#[repr(C)]
pub union NsStringData {
    pub inline_data: [u8; 24],
    pub heap: ManuallyDrop<NsStringHeap>,
}

// Final string struct
#[repr(C)]
pub struct NsString {
    pub len: usize,
    pub is_heap: bool,
    pub data: NsStringData,
}

/// Creates a new NsString from a C string.
///
/// # Safety
///
/// * `dest` must point to a valid, properly aligned `NsString` struct.
/// * `c_str` must be a valid, null-terminated C string, or a null pointer.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_string_new(dest: *mut NsString, c_str: *const c_char) -> NsError {
    if dest.is_null() {
        return NsError::Any;
    }

    if c_str.is_null() {
        unsafe {
            *dest = NsString {
                len: 0,
                is_heap: false,
                data: NsStringData {
                    inline_data: [0; 24],
                },
            };
        }
        return NsError::Success;
    }

    let rust_str = unsafe { CStr::from_ptr(c_str) }.to_bytes();
    let len = rust_str.len();

    if len < 24 {
        let mut inline = [0; 24];
        inline[..len].copy_from_slice(rust_str);

        unsafe {
            *dest = NsString {
                len,
                is_heap: false,
                data: NsStringData {
                    inline_data: inline,
                },
            };
        }
        NsError::Success
    } else {
        let mut vec = Vec::new();
        if vec.try_reserve(len).is_err() {
            return NsError::StringAllocFailed;
        }

        vec.extend_from_slice(rust_str);
        vec.shrink_to_fit();

        let ptr = vec.as_mut_ptr();
        let capacity = vec.capacity();
        std::mem::forget(vec);

        unsafe {
            *dest = NsString {
                len,
                is_heap: true,
                data: NsStringData {
                    heap: ManuallyDrop::new(NsStringHeap { ptr, capacity }),
                },
            };
        }
        NsError::Success
    }
}

/// Concatenates two NsStrings safely.
///
/// # Safety
///
/// * `dest` must point to a valid, properly aligned `NsString` struct.
/// * `s1` and `s2` must be valid `NsString` structs initialized by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_string_concat(
    dest: *mut NsString,
    s1: NsString,
    s2: NsString,
) -> NsError {
    if dest.is_null() {
        return NsError::Any;
    }

    let bytes1 = unsafe {
        if s1.is_heap {
            std::slice::from_raw_parts(s1.data.heap.ptr, s1.len)
        } else {
            std::slice::from_raw_parts(s1.data.inline_data.as_ptr(), s1.len)
        }
    };

    let bytes2 = unsafe {
        if s2.is_heap {
            std::slice::from_raw_parts(s2.data.heap.ptr, s2.len)
        } else {
            std::slice::from_raw_parts(s2.data.inline_data.as_ptr(), s2.len)
        }
    };

    let total_len = bytes1.len() + bytes2.len();

    if total_len < 24 {
        let mut inline = [0; 24];
        inline[..bytes1.len()].copy_from_slice(bytes1);
        inline[bytes1.len()..total_len].copy_from_slice(bytes2);

        unsafe {
            *dest = NsString {
                len: total_len,
                is_heap: false,
                data: NsStringData {
                    inline_data: inline,
                },
            };
        }
        NsError::Success
    } else {
        let mut vec = Vec::new();
        if vec.try_reserve(total_len).is_err() {
            return NsError::StringAllocFailed;
        }

        vec.extend_from_slice(bytes1);
        vec.extend_from_slice(bytes2);
        vec.shrink_to_fit();

        let ptr = vec.as_mut_ptr();
        let capacity = vec.capacity();
        std::mem::forget(vec);

        unsafe {
            *dest = NsString {
                len: total_len,
                is_heap: true,
                data: NsStringData {
                    heap: ManuallyDrop::new(NsStringHeap { ptr, capacity }),
                },
            };
        }
        NsError::Success
    }
}

/// Frees the heap memory associated with an NsString, if any.
///
/// # Safety
///
/// * `s` must be a valid pointer to an `NsString` previously initialized by this library.
/// * This function must not be called more than once on the same heap-allocated string.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_string_free(s: *mut NsString) {
    if s.is_null() {
        return;
    }

    let s_ref = unsafe { &mut *s };

    if s_ref.is_heap {
        unsafe {
            let heap = &s_ref.data.heap;
            let _ = Vec::from_raw_parts(heap.ptr, s_ref.len, heap.capacity);
        }
    }

    s_ref.len = 0;
    s_ref.is_heap = false;
}
