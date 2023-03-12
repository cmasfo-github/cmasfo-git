
const HELP: &str = "
open:
  open <port>: open the localhost with the port <port>
    e.g. 'open 8888'
  open <url>: open the url with the url <url>
    e.g. 'open google.com'
  open <word>: open the related web, if it's supported
    e.g. 'open google'
silly:
  enter chat mode to silly, which is a silly chatbot
";

pub(super) fn help() {

  println!("{}", HELP);

}
