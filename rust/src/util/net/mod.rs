
pub enum FileBuffer {
  Text(String),
  Binary(Vec<u8>),
}

pub struct HttpResult {
  // filename, not considering the url
  filename: Option<String>,
  // save as string if it's a page
  // save as binary if it's a file
  filebuffer: Option<FileBuffer>,
}

pub fn http_get(url: &str) -> Result<HttpResult, String> {
  todo!()
}

pub struct HttpFile {
  // url including filename
  // filename will also be defined here
  pub target_url: String,
  // buffer to serve
  pub filebuffer: FileBuffer,
}

use std::collections::HashMap;

pub struct HttpServer {
  port: u16,
  // target url and file buffer
  files: HashMap<String, HttpFile>,
}

impl HttpServer {
  // return an error if there is already a same url
  pub fn add_file(&mut self, file: &HttpFile) -> Result<(), String> {
    todo!()
  }
  // 404 file
  pub fn add_404(&mut self, file: &HttpFile) -> Result<(), String> {
    todo!()
  }
  // this will make a thread and start running the server!
  // the site is based one the 'files' hashmap
  pub fn start_running(&mut self) -> Result<(), String> {
    todo!()
  }
  // Ok(true): server stopped
  // Ok(false): server already stopped
  // Err(msg): failed to stop running
  pub fn stop_running(&mut self) -> Result<(), String> {
    todo!()
  }
}
