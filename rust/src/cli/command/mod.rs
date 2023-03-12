
use crate::*;

pub(super) fn cmd_open(cmd: &Vec<String>) {

  if let Some(url) = cmd.get(1) {
    open(url);
  }

}
