
mod get_right;
pub use get_right::*;

mod rand;

// get a line from the user
pub fn get_line() -> String {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  line.trim().to_string()
}

// print without line changing
#[macro_export]
macro_rules! printfl {
  ($($arg:tt)*) => ({
    print!($($arg)*);
    use std::io::Write;
    std::io::stdout().flush().unwrap();
  });
}

// flush the buffer (use this with 'print!')
pub fn flush() {
  use std::io::Write;
  std::io::stdout().flush().unwrap();
}

// flush the buffer then change the line
pub fn flushln() {
  println!(); // an empty println
}

// print a message, then get a line
#[macro_export]
macro_rules! msg_line {
  ($($arg:tt)*) => {{
    printfl!($($arg)*);
    get_line()
  }};
}
