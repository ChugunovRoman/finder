#[cfg(test)]
extern crate tempdir;

use std::{
  fs,
  path::{Path, PathBuf},
};
#[cfg(test)]
use tempdir::TempDir;

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
    let files: Vec<String> = vec![
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "image.jpg"))
    ];

    for file in &files {
      fs::write(file, "").unwrap();
    }

    files
  }

  pub fn some_files(&self) -> Vec<String> {
    let files: Vec<String> = vec![
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "image.jpg")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial Italic.ttf")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial bold.ttf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Arial.ttc")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Cuber.otf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Im.otf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "logo.jpeg")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "lol.png")),
    ];

    for file in &files {
      fs::write(file, "").unwrap();
    }

    files
  }

  pub fn one_depth(&self) -> Vec<String> {
    let paths: Vec<String> = vec![
      format!("{}/{}", self.root.to_str().unwrap(), "images")
    ];
    let files: Vec<String> = vec![
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/image.jpg")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial Italic.ttf")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial bold.ttf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Arial.ttc")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Cuber.otf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Im.otf")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/logo.jpeg")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/lol.png")),
    ];

    for path in &paths {
      fs::create_dir_all(path).unwrap();
    }

    for file in &files {
      fs::write(file, "").unwrap();
    }

    files
  }

  pub fn two_depth(&self) -> Vec<String> {
    let paths: Vec<String> = vec![
      format!("{}/{}", self.root.to_str().unwrap(), "images/meme"),
    ];
    let files: Vec<String> = vec![
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/image.jpg")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial Italic.ttf")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial bold.ttf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Arial.ttc")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Cuber.otf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Im.otf")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/logo.jpeg")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/lol.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/bird.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/ThisIsFine.png")),
    ];

    for path in &paths {
      fs::create_dir_all(path).unwrap();
    }

    for file in &files {
      fs::write(file, "").unwrap();
    }

    files
  }

  pub fn five_depth(&self) -> Vec<String> {
    let paths: Vec<String> = vec![
      format!("{}/{}", self.root.to_str().unwrap(), "images/meme/3/4/5"),
    ];
    let files: Vec<String> = vec![
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/image.jpg")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial Italic.ttf")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "Arial bold.ttf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Arial.ttc")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Cuber.otf")),
      String::from(format!("{}/{}", self.root.to_str().unwrap(), "Im.otf")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/logo.jpeg")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/3/lol.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/3/4/bird.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/3/4/bird1.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/3/4/bird2.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/3/4/5/ThisIsFine.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/3/4/5/meme1.png")),
      String::from(format!("{}/{}",self.root.to_str().unwrap(), "images/meme/3/4/5/meme2.png")),
    ];

    for path in &paths {
      fs::create_dir_all(path).unwrap();
    }

    for file in &files {
      fs::write(file, "").unwrap();
    }

    files
  }

  pub fn dataset_1(&self) -> Vec<String> {
    let mut files: Vec<String> = vec![];
    let paths: Vec<String> = vec![
      format!("{}/{}", self.root.to_str().unwrap(), "fonts"),
      format!("{}/{}", self.root.to_str().unwrap(), "fonts/truetype"),
      format!("{}/{}", self.root.to_str().unwrap(), "fonts/opentype"),
      format!("{}/{}", self.root.to_str().unwrap(), "fonts/X11"),
      format!("{}/{}", self.root.to_str().unwrap(), "fonts/cmap"),
      format!("{}/{}", self.root.to_str().unwrap(), "fonts/font-awesome"),
      format!("{}/{}", self.root.to_str().unwrap(), "fonts/type1"),
      format!("{}/{}", self.root.to_str().unwrap(), "fonts/users"),
    ];

    for path in &paths {
      fs::create_dir_all(path).unwrap();

      for index in 0..100 {
        fs::write(format!("{}/file_{}.ttf", &path, &index), "").unwrap();

        files.push(format!("{}/file_{}.ttf", &path, &index));
      }
    }

    files
  }
}
