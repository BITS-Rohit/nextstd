use ns_error::NsError;
use ns_string::NsString;
use std::ffi::{CString, c_double, c_float, c_int};
use std::io::{self, Write};
use std::os::raw::c_char;

// Helper function to read a line from stdin safely
fn read_line_buffer() -> Result<String, NsError> {
    let mut buffer = String::new();

    // If reading from stdin completely fails (e.g., broken pipe), return the specific error
    if io::stdin().read_line(&mut buffer).is_err() {
        return Err(NsError::IoReadFailed);
    }

    Ok(buffer.trim().to_string())
}

/// Read Integer
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer provided by C.
/// The caller must ensure that `ptr` is valid, properly aligned, and points to
/// initialized memory of type `c_int`. The function performs a null check, but
/// cannot guarantee the pointer is not dangling.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_int(ptr: *mut c_int) -> NsError {
    if ptr.is_null() {
        return NsError::Any; // Prevents segfaults, flags an error
    }

    match read_line_buffer() {
        Ok(input) => {
            // Match the parse result instead of blindly unwrapping!
            match input.parse::<c_int>() {
                Ok(val) => {
                    unsafe {
                        *ptr = val;
                    }
                    NsError::Success
                }
                Err(_) => NsError::InvalidInput,
            }
        }
        Err(e) => e,
    }
}

/// Read float
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer provided by C.
/// The caller must ensure that `ptr` is valid, properly aligned, and points to
/// initialized memory of type `c_float`. The function performs a null check, but
/// cannot guarantee the pointer is not dangling.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_float(ptr: *mut c_float) -> NsError {
    if ptr.is_null() {
        return NsError::Any;
    }

    match read_line_buffer() {
        Ok(input) => match input.parse::<c_float>() {
            Ok(val) => {
                unsafe {
                    *ptr = val;
                }
                NsError::Success
            }
            Err(_) => NsError::InvalidInput,
        },
        Err(e) => e,
    }
}

/// Read double
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer provided by C.
/// The caller must ensure that `ptr` is valid, properly aligned, and points to
/// initialized memory of type `c_double`. The function performs a null check, but
/// cannot guarantee the pointer is not dangling.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_double(ptr: *mut c_double) -> NsError {
    if ptr.is_null() {
        return NsError::Any;
    }

    match read_line_buffer() {
        Ok(input) => match input.parse::<c_double>() {
            Ok(val) => {
                unsafe {
                    *ptr = val;
                }
                NsError::Success
            }
            Err(_) => NsError::InvalidInput,
        },
        Err(e) => e,
    }
}

/// Read Boolean
///
/// # Safety
///
/// Reads Boolean values
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_bool(out: *mut bool) -> NsError {
    if out.is_null() {
        return NsError::Any;
    }

    let mut input = String::new();

    // Flush stdout to ensure any previous prints are visible
    let _ = io::stdout().flush();

    if io::stdin().read_line(&mut input).is_err() {
        return NsError::IoReadFailed;
    }

    // Safely parse the input, allowing for friendly terms like "true", "yes", or "1"
    match input.trim().to_lowercase().as_str() {
        "true" | "t" | "yes" | "y" | "1" => {
            unsafe {
                *out = true;
            } // <-- Wrapped in unsafe block
            NsError::Success
        }
        "false" | "f" | "no" | "n" | "0" => {
            unsafe {
                *out = false;
            } // <-- Wrapped in unsafe block
            NsError::Success
        }
        _ => NsError::InvalidInput,
    }
}

unsafe extern "C" {
    fn ns_string_new(dest: *mut NsString, c_str: *const c_char) -> NsError;
}

/// Read NextStd String
///
/// # Safety
///
/// This function reads dynamic input and routes it through
/// NextStd's native string initializer to ensure SSO
/// and heap boundaries are respected safetly
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_read_ns_string(ptr: *mut NsString) -> NsError {
    if ptr.is_null() {
        return NsError::Any;
    }

    match read_line_buffer() {
        Ok(input) => match CString::new(input) {
            Ok(c_str) => unsafe { ns_string_new(ptr, c_str.as_ptr()) },
            Err(_) => NsError::InvalidInput,
        },
        Err(e) => e,
    }
}
