use ns_error::NsError;

#[repr(C)]
pub struct NsVec {
    pub data: *mut u8,
    pub len: usize,
    pub capacity: usize,
    pub element_size: usize,
}

/// Initialize a new dynamic array
///
/// # Safety
///
/// * `dest` must point to a valid, properly aligned `NsVec` struct
/// * `element_size` must be greater than 0
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_vec_new(dest: *mut NsVec, element_size: usize) -> NsError {
    if dest.is_null() {
        return NsError::Any;
    }

    // Start with a conservative capacity to avoid immediate reallocation
    let initial_capacity = 4;
    let bytes_to_allocate = initial_capacity * element_size;

    let mut vec = Vec::<u8>::new();

    if vec.try_reserve_exact(bytes_to_allocate).is_err() {
        return NsError::Any;
    }

    unsafe {
        *dest = NsVec {
            data: vec.as_mut_ptr(),
            len: 0,
            capacity: initial_capacity,
            element_size,
        };
    }

    std::mem::forget(vec);

    NsError::Success
}

/// Doubles the capacity of the vector when the function exits
///
/// # Safety
///
/// * `v` must point to a valid `NsVec` initialized by `ns_vec_new`
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_vec_grow(v: *mut NsVec) -> NsError {
    if v.is_null() {
        return NsError::Any;
    }

    let vec_ref = unsafe { &mut *v };

    if vec_ref.capacity == 0 || vec_ref.element_size == 0 {
        return NsError::Any;
    }

    let current_bytes = vec_ref.capacity * vec_ref.element_size;
    let new_capacity = vec_ref.capacity * 2;
    let new_bytes = new_capacity * vec_ref.element_size;
    let bytes_to_add = new_bytes - current_bytes;

    unsafe {
        let mut vec = Vec::from_raw_parts(
            vec_ref.data,
            vec_ref.len * vec_ref.element_size,
            current_bytes,
        );

        if vec.try_reserve_exact(bytes_to_add).is_err() {
            std::mem::forget(vec);
            return NsError::Any;
        }

        vec_ref.data = vec.as_mut_ptr();
        vec_ref.capacity = new_capacity;

        std::mem::forget(vec);
    }

    NsError::Success
}

/// Frees the Heap Memory associated with NsVec
///
/// # Safety
///
/// * `v` must be a valid pointer to an `NsVec` previously initialized by this library
/// * This function must not be called more than once in the same vector
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_vec_free(v: *mut NsVec) {
    if v.is_null() {
        return;
    }

    let vec_ref = unsafe { &mut *v };

    if vec_ref.capacity > 0 && !vec_ref.data.is_null() {
        unsafe {
            let current_bytes = vec_ref.capacity * vec_ref.element_size;
            let _ = Vec::from_raw_parts(vec_ref.data, 0, current_bytes);
        }
    }

    vec_ref.data = std::ptr::null_mut();
    vec_ref.len = 0;
    vec_ref.capacity = 0;
    vec_ref.element_size = 0;
}
