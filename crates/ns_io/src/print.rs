use ns_string::NsString;
use std::ffi::CStr;
use std::io::{self, Write};
use std::mem::ManuallyDrop;
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

// String Interpolation

#[repr(C)]
pub enum NsTypeTag {
    Int = 0,
    Float,
    Double,
    Bool,
    SizeT,
    CStr,
    NsString,
}

#[repr(C)]
pub union NsAnyData {
    pub v_int: i32,
    pub v_float: f32,
    pub v_double: f64,
    pub v_bool: bool,
    pub v_size_t: usize,
    pub v_cstr: *const c_char,
    pub v_ns_string: ManuallyDrop<NsString>,
}

#[repr(C)]
pub struct NsAnyT {
    pub tag: NsTypeTag,
    pub data: NsAnyData,
}

// Formatter Engine
// Takes the format string and array of arguments, and builds the final string
fn format_any(fmt_str: &str, args_slice: &[NsAnyT]) -> String {
    let mut result = String::new();
    let mut arg_idx = 0;

    // String splitting
    let mut parts = fmt_str.split("{}");

    if let Some(first) = parts.next() {
        result.push_str(first);
    }

    for part in parts {
        if arg_idx < args_slice.len() {
            let arg = &args_slice[arg_idx];

            let formatted_val = unsafe {
                match arg.tag {
                    NsTypeTag::Int => arg.data.v_int.to_string(),
                    NsTypeTag::Float => arg.data.v_float.to_string(),
                    NsTypeTag::Double => arg.data.v_double.to_string(),
                    NsTypeTag::Bool => {
                        if arg.data.v_bool {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        }
                    }
                    NsTypeTag::SizeT => arg.data.v_size_t.to_string(),
                    NsTypeTag::CStr => {
                        if arg.data.v_cstr.is_null() {
                            "(null)".to_string()
                        } else {
                            CStr::from_ptr(arg.data.v_cstr)
                                .to_string_lossy()
                                .into_owned()
                        }
                    }
                    NsTypeTag::NsString => {
                        let val = &*arg.data.v_ns_string;
                        let bytes: &[u8] = if val.is_heap {
                            std::slice::from_raw_parts(val.data.heap.ptr as *const u8, val.len)
                        } else {
                            std::slice::from_raw_parts(val.data.inline_data.as_ptr(), val.len)
                        };
                        String::from_utf8_lossy(bytes).into_owned()
                    }
                }
            };

            result.push_str(&formatted_val);
            arg_idx += 1;
        } else {
            result.push_str("{}");
        }
        result.push_str(part);
    }

    result
}

// FFI boundary functions
#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn ns_print_fmt_c(fmt: *const c_char, args: *const NsAnyT, num_args: usize) {
    if fmt.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(fmt) };

    let fmt_str = c_str.to_string_lossy();

    let args_slice = unsafe { std::slice::from_raw_parts(args, num_args) };

    let final_output = format_any(&fmt_str, args_slice);

    print!("{}", final_output);
    io::stdout().flush().unwrap();
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn ns_println_fmt_c(fmt: *const c_char, args: *const NsAnyT, num_args: usize) {
    if fmt.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(fmt) };

    let fmt_str = c_str.to_string_lossy();

    let args_slice = unsafe { std::slice::from_raw_parts(args, num_args) };

    let final_output = format_any(&fmt_str, args_slice);

    println!("{}", final_output);
}
