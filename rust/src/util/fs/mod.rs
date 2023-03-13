
// 'fs' means 'file system'

pub fn read_to_string<T: AsRef<Path>>(path: T)
-> std::io::Result<String> {
  std::fs::read_to_string(path)
}

pub fn read_to_buffer<T: AsRef<Path>>(path: T)
-> std::io::Result<Vec<u8>> {
  std::fs::read(path)
}

pub fn write_as_string<T: AsRef<Path>>(path: T, contents: &str)
-> std::io::Result<()> {
  std::fs::write(path, contents)
}

pub fn write_as_buffer<T: AsRef<Path>>(path: T, contents: &Vec<u8>)
-> std::io::Result<()> {
  std::fs::write(path, contents)
}

use std::path::{Path, PathBuf};
use std::collections::HashMap;

// fs path wrapper
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FsPath {
  path: PathBuf,
}

use std::ffi::OsStr;
impl<T: ?Sized + AsRef<OsStr>> From<&T> for FsPath {
  #[inline]
  fn from(s: &T) -> FsPath {
    FsPath {
      path: PathBuf::from(s)
    }
  }
}

impl FsPath {
  pub fn as_path<'a>(&'a self) -> &'a Path {
    self.path.as_path()
  }
  pub fn to_pathbuf(&self) -> PathBuf {
    self.path.clone()
  }
  pub fn as_str(&self) -> &str {
    self.path.to_str().unwrap()
  }
  pub fn to_string(&self) -> String {
    self.path.to_str().unwrap().to_string()
  }
}

#[derive(PartialEq)]
pub enum FsBuffer {
  String(String),
  Buffer(Vec<u8>),
}

#[derive(PartialEq)]
pub enum FsType {
  File(Option<FsBuffer>),
  Dir,
  SymLink(Option<FsPath>),
  Unknown,
}

pub struct FsEntry {
  fspath: FsPath,
  fstype: Option<FsType>,
}

impl FsEntry {
  
  pub fn init_type(&mut self) -> std::io::Result<()> {
    match std::fs::metadata(&self.fspath.path) {
      Ok(meta) => {
        if meta.is_file() {
          self.fstype = Some(FsType::File(None));
        } else if meta.is_dir() {
          self.fstype = Some(FsType::Dir);
        } else if meta.is_symlink() {
          self.fstype = Some(FsType::SymLink(None));
        } else {
          self.fstype = Some(FsType::Unknown);
        }
        Ok(())
      }
      Err(e) => {
        Err(e)
      }
    }
  }
  
  pub fn is_file(&self) -> bool {
    self.fspath.as_path().is_file()
  }
  
  pub fn is_dir(&self) -> bool {
    self.fspath.as_path().is_dir()
  }
  
  pub fn is_symlink(&self) -> bool {
    self.fspath.as_path().is_symlink()
  }
  
  pub fn init(&mut self) -> std::io::Result<()> {
    if self.fstype.is_none() { self.init_type()?; }
    
    if self.is_file() {
      if self.fstype == Some(FsType::File(None)) {
        if let Ok(s) = read_to_string(&self.fspath.path) {
          let buffer = FsBuffer::String(s);
          let file = FsType::File(Some(buffer));
          self.fstype = Some(file);
        } else {
          match read_to_buffer(&self.fspath.path) {
            Ok(b) => {
              let buffer = FsBuffer::Buffer(b);
              let file = FsType::File(Some(buffer));
              self.fstype = Some(file);
            }
            Err(e) => {
              return Err(e);
            }
          }
        }
      }
    } else if self.is_symlink() {
      self.fstype = Some(FsType::SymLink(
        Some(FsPath {
          path: std::fs::read_link(&self.fspath.path)?
        })
      ));
    } // nothing for dir and unknown
    
    Ok(())
  }
  
}

// this helps handling multiple entries
pub struct FsHandle {
  entries: HashMap<FsPath, FsEntry>,
}

impl<'a> FsHandle {
  pub fn iter(&'a self)
  -> std::collections::hash_map::Iter<'a, FsPath, FsEntry> {
    self.entries.iter()
  }
  pub fn iter_mut(&'a mut self)
  -> std::collections::hash_map::IterMut<'a, FsPath, FsEntry> {
    self.entries.iter_mut()
  }
}

impl FsHandle {

  pub fn new() -> FsHandle {
    FsHandle {
      entries: HashMap::new(),
    }
  }

  pub fn from(path: &str)
  -> std::io::Result<FsHandle> {
    let mut fs = FsHandle::new();
    fs.scan_recursive(path)?;
    Ok(fs)
  }
  
  // scan: just scan entry list, not loading
  // root: scan in non-recursive way
  pub fn scan_root(&mut self, path: &str)
  -> std::io::Result<()> {
    for entry in std::fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();
      let fspath = FsPath::from(&path);
      let entry = FsEntry {
        fspath: fspath.clone(),
        fstype: None,
      };
      self.entries.insert(fspath, entry);
    }
    Ok(())
  }
  
  // scan in recursive way
  pub fn scan_recursive(&mut self, path: &str)
  -> std::io::Result<()> {
    for entry in std::fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();
      let fspath = FsPath::from(&path);
      let entry = FsEntry {
        fspath: fspath.clone(),
        fstype: None,
      };
      self.entries.insert(fspath, entry);
      if path.is_dir() {
        self.scan_recursive(path.to_str().unwrap())?;
      }
    }
    Ok(())
  }
  
  // load scanned entries
  pub fn init(&mut self)
  -> std::io::Result<()> {
    for (_, v) in self.entries.iter_mut() {
      v.init()?
    }
    Ok(())
  }
  
  // load with additional scanning
  pub fn init_recursive(&mut self)
  -> std::io::Result<()> {
    let mut root = std::mem::replace(&mut self.entries, HashMap::new());
    
    for (k, mut v) in root.drain() {
      v.init()?;
      if v.is_dir() {
        let path = v.fspath.as_str();
        self.load_recursive(path)?;
      }
      self.entries.insert(k, v);
    }

    Ok(())
  }
  
  // scan and load, in non-recursive way
  pub fn load_root(&mut self, path: &str)
  -> std::io::Result<()> {
    self.scan_root(path)?;
    self.init()
  }
  
  // scan and load, in recursive way
  pub fn load_recursive(&mut self, path: &str)
  -> std::io::Result<()> {
    self.scan_root(path)?;
    self.init_recursive()
  }
  
  pub fn get_entry(&self, path: &str) -> Option<&FsEntry> {
    self.entries.get(&FsPath::from(path))    
  }
  
  pub fn get_mut_entry(&mut self, path: &str) -> Option<&mut FsEntry> {
    self.entries.get_mut(&FsPath::from(path))
  }
  
  pub fn add_entry(&mut self, path: &str, entry: FsEntry) {
    self.entries.insert(FsPath::from(path), entry);
  }
  
  pub fn del_entry(&mut self, path: &str) {
    self.entries.remove(&FsPath::from(path));
  }
  
  // take the exact one entry which matches the path
  pub fn take_entry(&mut self, path: &str) -> Option<FsEntry> {
    self.entries.remove(&FsPath::from(path))
  }
  
  // take all entries which its path
  // starts with the given path
  pub fn take_entries(&mut self, path: &str) -> Option<FsHandle> {
    let mut legacy_map = std::mem::replace(&mut self.entries, HashMap::new());
    let mut return_map = HashMap::new();
    
    for (key, value) in legacy_map.drain() {
      if key.to_string().starts_with(path) {
        return_map.insert(key, value);
      } else {
        self.entries.insert(key, value);
      }
    }
    
    if return_map.is_empty() {
      None
    } else {
      Some(FsHandle { entries: return_map })
    }
  }

  pub fn print_entries(&self) {
    for (k, _) in self.entries.iter() {
      println!("{}", k.as_str());
    }
  }

  pub fn print_sorted(&self) {
    let mut vec = Vec::new();
    for (k, _) in self.entries.iter() {
      vec.push(k.as_str());
    }
    vec.sort();
    for k in vec {
      println!("{}", k);
    }
  }
  
}
