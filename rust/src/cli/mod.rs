
use crate::*;

pub struct Cli {

}

impl Cli {

  pub fn run() {
    let _line = msg_line!("Command: ");
    get_right::<u32>("Type u32: ", "Again.");
  }

}
