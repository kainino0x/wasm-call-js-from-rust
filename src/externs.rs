use std::ffi::CString;

/// Safe rust wrapper for our JS function `alert`.
pub fn alert(x: &str) {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::alert(ptr) }
}

/// Safe rust wrapper for emscripten_run_script_int (basically, JS eval()).
pub fn eval(x: &str) -> i32 {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::emscripten_run_script_int(ptr) }
}

// This is mostly standard Rust-C FFI stuff.
mod ffi {
    use libc::*;

    extern "C" {
        // This extern is defined in `html/library.js`.
        pub fn alert(x: *const c_char);
        // This extern is built in by Emscripten.
        pub fn emscripten_run_script_int(x: *const c_char) -> c_int;
    }
}
