use ns_string::NsString;
use std::ffi::CStr;
use std::io::{self, Write};
use std::os::raw::c_char;

// No Newline functions

// Print Integer
#[unsafe(no_mangle)]
pub extern "C" fn ns_print_int(val: i32) {
    // Adds newline by default
    print!("{}", val);
    io::stdout().flush().unwrap();
}

// Print Float
#[unsafe(no_mangle)]
pub extern "C" fn ns_print_float(val: f32) {
    // Adds newline by default
    print!("{}", val);
    io::stdout().flush().unwrap();
}

// Print Double
#[unsafe(no_mangle)]
pub extern "C" fn ns_print_double(val: f64) {
    // Adds newline by default
    print!("{}", val);
    io::stdout().flush().unwrap();
}

// Print String
#[unsafe(no_mangle)]
// If the C string is not null terminaltes '\0' , the function will keep on reading memory until
// the program crashes (Segfault)
// The below suppression is to prevent that
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn ns_print_string(ptr: *const c_char) {
    // Never Dereference a NULL pointer
    if ptr.is_null() {
        print!("(null)");
        io::stdout().flush().unwrap();
        return;
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };

    // Converting to Rust String
    // "to_string_lossy()" is best if the string has non-UTF-8 characters
    // Replaces them with <SPACE> instead of crashing
    print!("{}", c_str.to_string_lossy());
    io::stdout().flush().unwrap();
}

// Newline functions
// Print Integer
#[unsafe(no_mangle)]
pub extern "C" fn ns_println_int(val: i32) {
    // Adds newline by default
    println!("{}", val);
}

// Print Float
#[unsafe(no_mangle)]
pub extern "C" fn ns_println_float(val: f32) {
    // Adds newline by default
    println!("{}", val);
}

// Print Double
#[unsafe(no_mangle)]
pub extern "C" fn ns_println_double(val: f64) {
    // Adds newline by default
    println!("{}", val);
}

// Print String
#[unsafe(no_mangle)]
// If the C string is not null terminaltes '\0' , the function will keep on reading memory until
// the program crashes (Segfault)
// The below suppression is to prevent that
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn ns_println_string(ptr: *const c_char) {
    // Never Dereference a NULL pointer
    if ptr.is_null() {
        println!("(null)");
        return;
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };

    // Converting to Rust String
    // "to_string_lossy()" is best if the string has non-UTF-8 characters
    // Replaces them with <SPACE> instead of crashing
    println!("{}", c_str.to_string_lossy());
}

// Printing ns_string types
// No newline
#[unsafe(no_mangle)]
pub extern "C" fn ns_print_ns_string(val: NsString) {
    let bytes: &[u8] = unsafe {
        if val.is_heap {
            std::slice::from_raw_parts(val.data.heap.ptr as *const u8, val.len)
        } else {
            std::slice::from_raw_parts(val.data.inline_data.as_ptr(), val.len)
        }
    };
    // COnvert to rust string (To handle non-UTF-8)
    let rust_str = String::from_utf8_lossy(bytes);

    // Print and flush
    print!("{}", rust_str);
    io::stdout().flush().unwrap();
}

// Newline
#[unsafe(no_mangle)]
pub extern "C" fn ns_println_ns_string(val: NsString) {
    let bytes: &[u8] = unsafe {
        if val.is_heap {
            std::slice::from_raw_parts(val.data.heap.ptr as *const u8, val.len)
        } else {
            std::slice::from_raw_parts(val.data.inline_data.as_ptr(), val.len)
        }
    };

    // COnvert to rust string (To handle non-UTF-8)
    let rust_str = String::from_utf8_lossy(bytes);

    // Print and flush
    println!("{}", rust_str);
}

// Boolean
/// No Newline
#[unsafe(no_mangle)]
pub extern "C" fn ns_print_bool(val: bool) {
    if val {
        print!("true");
    } else {
        print!("false");
    }
}

/// Newline
#[unsafe(no_mangle)]
pub extern "C" fn ns_println_bool(val: bool) {
    if val {
        println!("true");
    } else {
        println!("false");
    }
}

// Printing size_t
// No Newline
#[unsafe(no_mangle)]
pub extern "C" fn ns_print_size_t(val: usize) {
    print!("{}", val);
    io::stdout().flush().unwrap();
}

// Newline
#[unsafe(no_mangle)]
pub extern "C" fn ns_println_size_t(val: usize) {
    println!("{}", val);
}
