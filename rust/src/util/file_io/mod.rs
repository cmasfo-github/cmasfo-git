
use std::path::{Path, PathBuf};
use std::fs::{File, DirEntry};

// just a path info
// use string instead of pathbuf
// because it's easier to modify
pub struct FsPath {
  path: Option<String>,
}

impl Clone for FsPath {
  fn clone(&self) -> FsPath {
    FsPath {
      path: self.path.clone(),
    }
  }
}

// util
impl FsPath {
  // this shouldn't be necessary to be public
  // if everything is working alright
  fn to_slash(&self) -> FsPath {
    if let Some(path) = &self.path {
      let path = path.replace("\\", "/");
      FsPath {
        path: Some(path),
      }
    } else {
      Self::new()
    }
  }
  // remove unnecessary "..", ".", or ""
  pub fn normalize(&self) -> FsPath {
    if let Some(path) = &self.path {
      let split = path.split("/");
      let mut vec = Vec::new();

      for word in split {
        if word == ".." {
          if vec.is_empty() {
            vec.push(word);
          } else {
            let last = *vec.last().unwrap();
            if last == ".." {
              vec.push(word);
            } else {
              vec.pop();
            }
          }
        } else if word == "." || word == "" {
          continue;
        } else {
          vec.push(word);
        }
      }

      let normalized = vec.join("/");

      return FsPath { path: Some(normalized) };
    }

    Self::new()
  }
  // use this after normalization
  pub fn is_over_root(&self) -> bool {
    if let Some(path) = &self.path {
      if path.starts_with("/..") || path.starts_with("..") {
        true
      } else {
        false
      }
    } else {
      false
    }
  }
  // check if it starts with slash
  pub fn is_begin_slash(&self) -> bool {
    if let Some(path) = &self.path {
      if path.starts_with("/") {
        true
      } else {
        false
      }
    } else {
      false
    }
  }
  // detach begin slash if there is
  pub fn off_begin_slash(&self) -> FsPath {
    if let Some(path) = &self.path {
      let path = path.trim_start_matches("/");
      FsPath {
        path: Some(path.to_string()),
      }
    } else {
      self.clone()
    }
  }
  // check if it ends with slash
  pub fn is_end_slash(&self) -> bool {
    if let Some(path) = &self.path {
      if path.ends_with("/") {
        true
      } else {
        false
      }
    } else {
      false
    }
  }
  // detach end slash if there is
  pub fn off_end_slash(&self) -> FsPath {
    if let Some(path) = &self.path {
      let path = path.trim_end_matches("/");
      FsPath {
        path: Some(path.to_string()),
      }
    } else {
      self.clone()
    }
  }
  // detach both slashes, if you want
  pub fn off_both_slash(&self) -> FsPath {
    if let Some(path) = &self.path {
      let path = path.trim_start_matches("/");
      let path = path.trim_end_matches("/");
      FsPath {
        path: Some(path.to_string()),
      }
    } else {
      self.clone()
    }
  }
  pub fn clip_front(&self, front: &str) -> Result<FsPath, String> {
    if let Some(path) = &self.path {
      if path.len() < front.len() {
        return Err(format!("Couldn't clip_front {} with {}", path, front));
      } else {
        if &path[0..front.len()] == front {
          let clipped = &path[front.len()..];
          return Ok(FsPath { path: Some(clipped.to_string()) });
        } else {
          return Err(format!("Couldn't clip_front {} with {}", path, front));
        }
      }
    } else {
      if front == "" {
        return Ok(Self::new());
      } else {
        return Err(format!("Couldn't clip_front the empty path with {}", front));
      }
    }
  }
  pub fn clip_behind(&self, behind: &str) -> Result<FsPath, String> {
    if let Some(path) = &self.path {
      if path.len() < behind.len() {
        return Err(format!("Couldn't clip_behind {} with {}", path, behind));
      } else {
        let idx = path.len() - behind.len();
        if &path[idx..] == behind {
          let clipped = &path[..idx];
          return Ok(FsPath { path: Some(clipped.to_string()) });
        } else {
          return Err(format!("Couldn't clip_behind {} with {}", path, behind));
        }
      }
    } else {
      if behind == "" {
        return Ok(Self::new());
      } else {
        return Err(format!("Couldn't clip_behind the empty path with {}", behind));
      }
    }
  }
  // the other comes after from self
  pub fn append(&self, other: &FsPath) -> FsPath {
    if let Some(path) = &self.path {
      if let Some(other) = &other.path {
        // both are some
        let path = path.to_string() + other;
        FsPath::from_str(&path)
      } else {
        // only self is some
        self.clone()
      }
    } else if other.path.is_some() {
      // only other is some
      other.clone()
    } else { // both are empty
      Self::new()
    }
  }
  // the other comes in front of self
  pub fn prepend(&self, other: &FsPath) -> FsPath {
    if let Some(path) = &self.path {
      if let Some(other) = &other.path {
        // both are some
        let path = other.to_string() + path;
        FsPath::from_str(&path)
      } else {
        // only self is some
        self.clone()
      }
    } else if other.path.is_some() {
      // only other is some
      other.clone()
    } else { // both are empty
      Self::new()
    }
  }
}

impl FsPath {
  fn new() -> FsPath {
    FsPath {
      path: None,
    }
  }
  fn from_str(path: &str) -> FsPath {
    FsPath {
      path: Some(path.to_string()),
    }.to_slash()
  }
  fn from_path(path: &Path) -> FsPath {
    FsPath {
      path: Some(path.to_string_lossy().to_string()),
    }.to_slash()
  }
  fn to_string(&self) -> String {
    self.path.clone().unwrap()
  }
  fn to_pathbuf(&self) -> PathBuf {
    PathBuf::from(self.to_string())
  }
}

pub enum FsType {
  Dir,
  File,
}

pub enum FsBuffer {
  Text(String),
  Binary(Vec<u8>),
}

// entry, which can be initialized
// however, the acutal file system
//  might have been changed after the initialization
// thus, this doesn't guarantee that it is still there
pub struct FsEntry {
  fspath: Option<FsPath>,
  fstype: Option<FsType>, // file or dir
  fsbuffer: Option<FsBuffer>,
}

impl FsEntry {
  pub fn new() -> FsEntry {
    FsEntry {
      fspath: None,
      fstype: None,
      fsbuffer: None,
    }
  }
  pub fn from_path(path: &FsPath) -> FsEntry {
    FsEntry {
      fspath: Some(path.clone()),
      fstype: None,
      fsbuffer: None,
    }
  }
  pub fn set_path(&mut self, path: &FsPath) {
    self.fspath = Some(path.clone());
  }
}

// bunch of entries
// this helps dealing with multiple entries
pub struct FsCollection {

}
