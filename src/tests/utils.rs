extern crate rand;

use rand::prelude::*;
use std::{
  fs,
  path::{Path, PathBuf},
};

pub struct DataSet {
  root: PathBuf,
  filter: Box<dyn Fn(String) -> bool>,
}

impl DataSet {
  pub fn new<P: AsRef<Path>>(path: P, filter: Box<dyn Fn(String) -> bool>) -> Self {
    DataSet {
      root: path.as_ref().to_path_buf(),
      filter
    }
  }

  pub fn gen(&self) -> Vec<String> {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";
    let extensions = vec![".otf", ".pdf", ".gz", ".ttf", ".ttc", ".png", ".jpeg"];
    let mut files = vec![];
    let mut rng = thread_rng();

    for _ in 0..20 {
      let dir_name: String = (0..10)
        .map(|_| {
          let index = rng.gen_range(0, charset.len());
          char::from(unsafe { *charset.get_unchecked(index) })
        })
        .collect();

      let depth = rng.gen_range(0, 10) - 1;
      let mut depth_path = vec![];

      for _ in 0..depth {
        let sub_dir_name: String = (0..10)
          .map(|_| {
            let index = rng.gen_range(0, charset.len());
            char::from(unsafe { *charset.get_unchecked(index) })
          })
          .collect();

        depth_path.push(sub_dir_name);
      }

      let dir = self.root.join(&dir_name).join(depth_path.join("/"));
      fs::create_dir_all(&dir).unwrap();

      for _ in 0..200 {
        let file_name: String = (0..15)
          .map(|_| {
            let index = rng.gen_range(0, charset.len());
            char::from(unsafe { *charset.get_unchecked(index) })
          })
          .collect();
        let ext = extensions.get(rng.gen_range(0, extensions.len())).unwrap();
        let file = format!("{}/{}{}", dir.to_str().unwrap(), file_name, ext);
        fs::write(&file, "").unwrap();

        files.push(file);
      }
    }

    files
  }
}
