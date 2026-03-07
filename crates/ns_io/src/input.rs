use std::io::{self};
use std::ffi::{c_int, c_float,c_double};

// Helper function to read a line from stdin safetly
fn read_line_buffer() -> String {
    let mut buffer = String::new();

    // Ignore error for now (eg:Closed pipe)
    let _ = io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

// Read Integer
#[unsafe(no_mangle)]
/// # Safety 
///
/// This function reads an integer input
pub unsafe extern "C" fn ns_read_int(ptr: *mut c_int) {
    if ptr.is_null() {
        return;
    }

    let input = read_line_buffer();

    // Try to parse. If it fails, default to 0
    let val: c_int = input.parse().unwrap_or(0);

    // Unsafe: Write value to C memory address 
    unsafe {
        *ptr = val;
    }
}

// Read float 
#[unsafe(no_mangle)]
/// # Safety 
///
/// This function reads a floating point input 
pub unsafe extern "C" fn ns_read_float(ptr: *mut c_float) {
    if ptr.is_null() {
        return;
    }

    let input = read_line_buffer();

    // Try to parse. If it fails, default to 0
    let val: c_float = input.parse().unwrap_or(0.0);

    // Unsafe: Write value to C memory address 
    unsafe {
        *ptr = val;
    }
}

// Read double 
#[unsafe(no_mangle)]
/// # Safety 
///
/// This function reads a double input 
pub unsafe extern "C" fn ns_read_double(ptr: *mut c_double) {
    if ptr.is_null() {
        return;
    }

    let input = read_line_buffer();

    // Try to parse. If it fails, default to 0
    let val: c_double = input.parse().unwrap_or(0.0);

    // Unsafe: Write value to C memory address 
    unsafe {
        *ptr = val;
    }
}

