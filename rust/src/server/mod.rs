
// this must serve the contents of 'site' module effectively
// to do that, this needs to serve pages and assets based on the url

use super::site::Site;

pub struct Server {
  site: Site,
  port: u16,
}

impl Server {
  pub fn new(site: Site, port: u16) -> Self {
    Server { site, port }
  }
  pub fn run(&mut self) {
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    let listener = TcpListener::bind(("127.0.0.1", self.port)).unwrap();
    
    for stream in listener.incoming() {
      
      let stream = stream.unwrap();

      // extract the url
      let url = stream.peer_addr().unwrap();

      

    }
  }
}
