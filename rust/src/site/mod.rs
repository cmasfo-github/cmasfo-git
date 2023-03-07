
// this whole module is about what pages to serve in gui mode
// it doesn't need to have some html files
// their contents just can be defined in the source code
// if asset files are necessary, place them in the asset dir

mod main;

use std::collections::HashMap;

pub enum AssetBuffer {
  Unknown, // not initialized yet
  Text(String), // document-style buffer
  Binary(Vec<u8>), // non-document buffer, like images
  Failed(String), // failed to have buffer, the string is the reason
}

impl Clone for AssetBuffer {
  fn clone(&self) -> Self {
    match self {
      AssetBuffer::Unknown => AssetBuffer::Unknown,
      AssetBuffer::Text(buf) => AssetBuffer::Text(buf.clone()),
      AssetBuffer::Binary(buf) => AssetBuffer::Binary(buf.clone()),
      AssetBuffer::Failed(err) => AssetBuffer::Failed(err.clone()),
    }
  }
}

pub struct Asset {
  id: String, // name, or id (both are same)
  filename: String, // specific url to serve
  filepath: String, // path of the actual file
  buffer: AssetBuffer, // the buffer of the file
}

pub struct Page {
  id: String, // name, or id (both are same)
  url: String, // specific url to serve
  title: String, // the tab title of the page
  content: String, // the html content of the page
}

// search with id, not url
// url is just for displaying
// for example, main page's id is "main"
// and it url is "" (empty)
pub trait TargetUrl {
  // insert
  fn id(&self) -> String;
  // search
  fn check_id(&self, name: &str) -> bool;
  // get after search
  fn url(&self) -> String;
  // get content
  fn content(&self) -> AssetBuffer;
}

impl TargetUrl for Page {
  fn id(&self) -> String {
    self.id.clone()
  }
  fn check_id(&self, id: &str) -> bool {
    self.id == id
  }
  fn url(&self) -> String {
    self.url.clone()
  }
  fn content(&self) -> AssetBuffer {
    AssetBuffer::Text(self.content.clone())
  }
}

impl TargetUrl for Asset {
  fn id(&self) -> String {
    self.id.clone()
  }
  fn check_id(&self, id: &str) -> bool {
    self.filename == id
  }
  fn url(&self) -> String {
    self.filename.clone()
  }
  fn content(&self) -> AssetBuffer {
    self.buffer.clone()
  }
}

pub struct Site {
  pages: HashMap<String, Page>, // name and page
  assets: HashMap<String, Asset>, // name and asset
}

impl Site {
  pub fn new() -> Site {
    // init the actual site here
    let mut pages = HashMap::new();
    let mut assets = HashMap::new();

    let page = main::main_page();
    pages.insert(page.id(), page);

    Site {
      pages,
      assets,
    }
  }
}
