use lib::rlib_run;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  println!("--- rlib Main ---");
  for (index, message) in args.iter().enumerate() {
    print!("{}: ", index);
    rlib_run(message);
  }
}

