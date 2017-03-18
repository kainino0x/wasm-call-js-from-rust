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

    // *ARCANE INCANTATION ALERT*
    // These arguments get passed to emcc (the Emscripten linker).
    // This tells the linker that these external symbols are not undefined.
    // (This is also why this project requires Rust Nightly. On stable, it
    // should be possible pass this directly to the linker instead, but I
    // think that means we can't use Cargo.)
    #[link_args = "--js-library html/library.js"]
    extern "C" {
        // This extern is defined in `html/library.js`.
        pub fn alert(x: *const c_char);
        // This extern is built in by Emscripten.
        pub fn emscripten_run_script_int(x: *const c_char) -> c_int;
    }
}
