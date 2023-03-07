
use crate::*;

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

// try few times, not infinitely
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
