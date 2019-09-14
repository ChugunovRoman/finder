#[cfg(test)]
extern crate tempdir;

use std::{
  fs,
  path::{Path, PathBuf},
};

pub struct DataSet {
  root: PathBuf,
}

impl DataSet {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    DataSet {
      root: path.as_ref().to_path_buf(),
    }
  }

  pub fn empty_dir(&self) -> Vec<String> {
    vec![]
  }
  pub fn one_file(&self) -> Vec<String> {
    let files: Vec<String> = vec![String::from(format!(
      "{}/{}",
      self.root.to_str().unwrap(),
      "image.jpg"
    ))];

    for file in &files {
      fs::write(file, "").unwrap();
    }

    files
  }
  pub fn some_files(&self, count: i32) {}
}
