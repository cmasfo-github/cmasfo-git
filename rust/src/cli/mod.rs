
use crate::*;

mod command;
use command::*;

mod nlp;
use nlp::*;

mod phind;
use phind::*;

mod silly;
use silly::*;

mod urban;
use urban::*;

mod help;
use help::*;

pub struct Cli {
  log: Vec<String>
}

impl Cli {

  //! Test doc

  /**
    Test doc
  */

  /// cli run code
  pub fn run() {
    
    println!("Welcome to Rust CLI App.");
    println!("To see a list of commands, type 'help'.");

    let mut cli = Cli {
      log: Vec::new()
    };

    loop {
      cli.msg_command("Command: ");
      match cli.process_command() {
        Continue::Yes => continue,
        Continue::No => break,
      }
    }

    println!("Goodbye.");

  }

}

impl Cli {

  fn get_command(&mut self) {
    let command = get_line();
    self.log.push(command);
  }

  fn msg_command(&mut self, msg: &str) {
    let command = msg_line!("{}", msg);
    self.log.push(command);
  }

}

enum Continue {
  Yes,
  No,
}

impl Cli {

  fn process_command(&mut self) -> Continue {

    let command = self.log.last().unwrap();
    let command = to_lower(command);
    let parsed = Self::parse_command(&command);

    if let Some(cmd) = parsed.get(0) {
      match cmd.as_str() {
        "help" => {
          help();
        }
        "version" => {
          println!("Version is not implemented yet.");
        }
        "open" => {
          cmd_open(&parsed);
        }
        "silly" => {
          mode_silly();
        }
        "sillybg" => {
          silly_bg();
        }
        "urban" => {
          mode_urban();
        }
        _ => {
          println!("Unknown command: {}", cmd);
        }
      }
    }

    Continue::Yes
    
  }

  fn parse_command(command: &str) -> Vec<String> {

    let split = command.split(' ');

    split.map(|s| s.to_string()).collect()

  }

}
