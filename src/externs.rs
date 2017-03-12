use std::ffi::CString;

pub fn alert(x: &str) {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::alert(ptr) }
}

pub fn eval(x: &str) -> i32 {
    let x = CString::new(x).unwrap();
    let ptr = x.as_ptr();
    unsafe { ffi::emscripten_run_script_int(ptr) }
}

mod ffi {
    use libc::*;

    #[link_args = "--js-library html/library.js"]
    extern "C" {
        pub fn alert(x: *const c_char);
        pub fn emscripten_run_script_int(x: *const c_char) -> c_int;
    }
}
