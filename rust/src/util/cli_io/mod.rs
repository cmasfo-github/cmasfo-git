
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

// repeat getting until getting it right
pub fn get_right<T>(
  msg_cmd: &str,
  msg_fail: &str
) -> T
where T: std::str::FromStr,
T::Err: std::fmt::Debug + std::fmt::Display {

  loop {

    printfl!("{msg_cmd}");

    let mut line = get_line();

    match line.parse::<T>() {
      Ok(t) => break t,
      Err(_) => {
        println!("{msg_fail}");
        continue;
      }
    };

  }

}

// try finite times
// return try limit for err
pub fn try_right<T>(
  msg_cmd: &str,
  msg_fail: &str,
  try_limit: u32,
) -> Result<T, u32>
where T: std::str::FromStr,
T::Err: std::fmt::Debug + std::fmt::Display {

  if try_limit == 0 {
    return Err(try_limit);
  }

  let mut tried = 0;

  loop {

    printfl!("{msg_cmd}");

    let mut line = get_line();

    match line.parse::<T>() {
      Ok(t) => break Ok(t),
      Err(_) => {
        tried += 1;
        if tried < try_limit {
          println!("{msg_fail}");
          continue;
        } else {
          break Err(try_limit);
        }
      }
    }

  }

}
