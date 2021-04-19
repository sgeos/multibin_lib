use std::ffi::CStr;
use std::os::raw::c_char;

// Converts FFI input and forwards it to a native Rust function.
#[no_mangle]
pub extern fn ffi_run(ffi_message: *const c_char) {
  let message: &str;
  unsafe {
    message = CStr::from_ptr(ffi_message).to_str().unwrap_or("");
  }
  rlib_run(message);
}

// A simple message printing function.
pub fn rlib_run(message: &str) {
  println!("{}", message);
}

