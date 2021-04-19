use lib::ffi_run;

use std::env;
use std::ffi::CString;

fn main() {
  // Map from String to FFI CString.
  let args: Vec<CString> = env::args()
    .map(|s| CString::new(s).expect("CString::new failed"))
    .collect();

  println!("--- FFI Main ---");
  for (index, message) in args.iter().enumerate() {
    print!("{}: ", index);
    ffi_run(message.as_ptr());
  }
}

