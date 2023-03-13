
// 'io' means 'input & output'

// necessary for flushing
use std::io::Write;

// println! (args)
// * good
// * nothing to fix

// print!
// * doesn't flush immediately
// * but it's still useful
//   if you are not done yet with that line
// printfl!
// * flushes immediately
// * useful for printing without changing the line
#[macro_export]
macro_rules! printfl {
  ($($arg:tt)*) => ({
    print!($($arg)*);
    std::io::stdout().flush().unwrap();
  })
}

// original flush
// * verbose
// * needs to use unwrap
// * needs to use trait (std::io::Write)
// new flush
// * short
// * doesn't need to use unwrap (but it still uses)
pub fn flush() {
  std::io::stdout().flush().unwrap();
}

// println! (no args)
// * not bad
// * but using the name 'println'
//   without args is a bit weird
// flushln
// * more consistent (with flush)
// * the purpose is more clear
pub fn flushln() {
  println!();
}

// read_line
// * verbose
// * requires to make a string
// * requires trimming
// * it appends the input to the string
// * appending sometimes causes problems
//   when the string is reused without being cleared
// * needs to use unwrap
// get_line
// * short
// * it just returns the string
// * doesn't need to trim
// * doesn't need to worry about appending
// * doesn't need to use unwrap (but it still uses)
pub fn get_line() -> String {
  let mut string = String::new();
  std::io::stdin().read_line(&mut string).unwrap();
  string.trim().to_string()
}

// printfl + get_line
// * a bit verbose
//   while they are being used very often
// msg_line!
// * convenient
// * gets a line with printing a message
#[macro_export]
macro_rules! msg_line {
  ($($arg:tt)*) => ({
    printfl!($($arg)*);
    get_line()
  })
}
