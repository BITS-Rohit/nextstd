use std::ffi::CStr;
use std::os::raw::c_char;
use std::io::{self, Write};

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
// The below supression is to prevent that 
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn ns_print_string(ptr: *const c_char) {
    // Never Dereference a NULL pointer
    if ptr.is_null() {
        print!("(null)");
        io::stdout().flush().unwrap();
        return;
    }

    let c_str = unsafe {
        CStr::from_ptr(ptr)
    };

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
// The below supression is to prevent that 
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn ns_println_string(ptr: *const c_char) {
    // Never Dereference a NULL pointer
    if ptr.is_null() {
        println!("(null)");
        return;
    }

    let c_str = unsafe {
        CStr::from_ptr(ptr)
    };

    // Converting to Rust String
    // "to_string_lossy()" is best if the string has non-UTF-8 characters
    // Replaces them with <SPACE> instead of crashing
    println!("{}", c_str.to_string_lossy());
}

