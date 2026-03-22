use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::os::raw::{c_char, c_int};
use std::process::Command;

use ns_error::NsError;

use ns_string::{NsString, NsStringData, NsStringHeap, ns_string_free};

#[repr(C)]
pub struct NsCmdOutput {
    pub stdout_data: NsString,
    pub stderr_data: NsString,
    pub exit_code: c_int,
}

fn rust_string_to_ns(s: String) -> NsString {
    let bytes = s.into_bytes();
    let len = bytes.len();

    if len < 24 {
        let mut inline = [0; 24];
        inline[..len].copy_from_slice(&bytes);
        NsString {
            len,
            is_heap: false,
            data: NsStringData {
                inline_data: inline,
            },
        }
    } else {
        let mut vec = bytes;
        vec.shrink_to_fit();
        let ptr = vec.as_mut_ptr();
        let capacity = vec.capacity();
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

// Extract a Rust &str from NextStd String Union
fn ns_to_rust_str(ns: &NsString) -> &str {
    let slice = unsafe {
        if ns.is_heap {
            std::slice::from_raw_parts(ns.data.heap.ptr, ns.len)
        } else {
            std::slice::from_raw_parts(ns.data.inline_data.as_ptr(), ns.len)
        }
    };

    // Invalid command returns empty string
    std::str::from_utf8(slice).unwrap_or("")
}

// CORE EXECUTION
fn execute_shell_command(cmd_str: &str, output: *mut NsCmdOutput) -> NsError {
    if output.is_null() {
        return NsError::Any;
    }

    // Spawn the process safely using the system shell
    let process_result = Command::new("sh").arg("-c").arg(cmd_str).output();

    match process_result {
        Ok(proc_output) => {
            let stdout_str = String::from_utf8_lossy(&proc_output.stdout).into_owned();
            let stderr_str = String::from_utf8_lossy(&proc_output.stderr).into_owned();

            unsafe {
                (*output).stdout_data = rust_string_to_ns(stdout_str);
                (*output).stderr_data = rust_string_to_ns(stderr_str);
                (*output).exit_code = proc_output.status.code().unwrap_or(-1);
            }
            NsError::Success
        }
        Err(_) => NsError::Any,
    }
}

// FFI exports
/// # Safety
///
/// Runs a command if the input is a *const char in C
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_cmd_run_cstr(
    command: *const c_char,
    output: *mut NsCmdOutput,
) -> NsError {
    if command.is_null() {
        return NsError::Any;
    }

    let cmd_str = unsafe { CStr::from_ptr(command) }.to_string_lossy();

    execute_shell_command(&cmd_str, output)
}

#[unsafe(no_mangle)]
pub extern "C" fn ns_cmd_run_ns(command: NsString, output: *mut NsCmdOutput) -> NsError {
    let cmd_str = ns_to_rust_str(&command);
    execute_shell_command(cmd_str, output)
}

/// # Safety
///
/// This function frees the strings in the output struct
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_cmd_output_free(output: *mut NsCmdOutput) {
    if output.is_null() {
        return;
    }

    unsafe {
        ns_string_free(&mut (*output).stdout_data);
        ns_string_free(&mut (*output).stderr_data);
    }
}
