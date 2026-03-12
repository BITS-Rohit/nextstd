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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn ns_string_new(c_str: *const c_char) -> NsString {
    if c_str.is_null() {
        return NsString {
            len: 0,
            is_heap: false,
            data: NsStringData {
                inline_data: [0; 24],
            },
        };
    }

    let rust_str = unsafe { CStr::from_ptr(c_str) }.to_bytes();

    let len = rust_str.len();

    // If it is less than 24 bytes pack it into the struct itself
    if len < 24 {
        let mut inline = [0; 24];
        inline[..len].copy_from_slice(rust_str);

        NsString {
            len,
            is_heap: false,
            data: NsStringData {
                inline_data: inline,
            },
        }
    } else {
        // If it is more than 24 bytes, put it on the heap
        let mut vec = rust_str.to_vec();

        vec.shrink_to_fit();

        let ptr = vec.as_mut_ptr();

        let capacity = vec.capacity();

        // Preventing rust from immediately freeing the vector memory
        std::mem::forget(vec);

        NsString {
            len,
            is_heap: true,
            data: NsStringData {
                heap: ManuallyDrop::new(NsStringHeap { ptr, capacity }),
            },
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn ns_string_free(s: *mut NsString) {
    if s.is_null() {
        return;
    }

    let s_ref = unsafe { &mut *s };

    if s_ref.is_heap {
        unsafe {
            let heap = &s_ref.data.heap;
            // Reclaim ownership of the memory and let it drop to free it
            let _ = Vec::from_raw_parts(heap.ptr, s_ref.len, heap.capacity);
        }
    }

    s_ref.len = 0;
    s_ref.is_heap = false;
}
