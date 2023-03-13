
use lib::*;

pub struct CliApp {

}

impl CliApp {

  pub fn run() {

    let mut fs = FsHandle::from("rust").unwrap();
    fs.init().unwrap();
    fs.print_sorted();

  }

}
