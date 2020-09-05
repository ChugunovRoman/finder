/*!
Crate `finder` is a very simple and lightweight file searcher with the filtering of files.
It provides an efficient implementation of recursive file search.

To use this crate, add `finder` as a dependency to your project's
`Cargo.toml`:

```toml
[dependencies]
finder = "0.1"
```

# Example

The following code recursively search all files in `/foo` and `/bar` diresctories:

```no_run
extern crate finder;

use finder::Finder;

fn main() {
  let finders = Finder::new("/foo:/bar");
  for i in finders.into_iter() {
    println!("{}", i.path().to_str().unwrap());
  }
}
```

# Example with filter

The following code recursively search `.ttf` and `.ttc` files in `/foo` and `/bar` diresctories:

```no_run
extern crate finder;

use std::fs::DirEntry;

use finder::Finder;

fn is_font_file(e: &DirEntry) -> bool {
  if let Some(s) = e.path().file_name() {
    let name = String::from(s.to_str().unwrap());

    if (name.ends_with(".ttf") || name.ends_with(".ttc")) {
      return true;
    }
  }

  false
}

fn main() {
  let finders = Finder::new("/foo:/bar");
  for i in finders.filter(&is_font_file).into_iter() {
    println!("{}", i.path().to_str().unwrap());
  }
}
```

*/

extern crate log;

mod tests;

use log::warn;
use std::fs::{self, DirEntry, ReadDir};
use std::io;
use std::path::{Path, PathBuf};

struct FinderOptions {
  filter: &'static Fn(&DirEntry) -> bool,
}

pub struct Finder {
  opts: FinderOptions,
  root: PathBuf,
}

impl Finder {
  pub fn new<P: AsRef<Path>>(root: P) -> Self {
    Finder {
      opts: FinderOptions { filter: &|_e| true },
      root: root.as_ref().to_path_buf(),
    }
  }

  pub fn filter(mut self, filter: &'static Fn(&DirEntry) -> bool) -> Self {
    self.opts.filter = filter;
    self
  }
}

impl IntoIterator for Finder {
  type Item = DirEntry;
  type IntoIter = IntoIter;

  fn into_iter(self) -> IntoIter {
    IntoIter {
      opts: self.opts,
      start: Some(self.root),
      entries: vec![],
    }
  }
}

pub struct IntoIter {
  opts: FinderOptions,
  start: Option<PathBuf>,
  entries: Vec<List>,
}

impl Iterator for IntoIter {
  type Item = DirEntry;

  fn next(&mut self) -> Option<DirEntry> {
    if self.entries.is_empty() {
      if let Some(start) = self.start.take() {
        for path in String::from(start.to_str().unwrap()).split(":") {
          self.handle_entry(&Path::new(path).to_path_buf());
        }
      }
    }

    while !self.entries.is_empty() {
      let next = self
        .entries
        .last_mut()
        .expect("BUG: entries should be non-empty")
        .next();
      match next {
        None => self.entries.pop(),
        Some(entry) => {
          let e = entry.unwrap();

          if e.path().is_dir() {
            self.handle_entry(&e.path());
          }

          if !e.path().is_dir() && (self.opts.filter)(&e) {
            return Some(e);
          }

          None
        }
      };
    }

    None
  }
}

impl IntoIter {
  pub fn handle_entry(&mut self, directory: &PathBuf) {
    warn!(
      "handle directory, is dir: {}, path: {}",
      directory.is_dir(),
      directory.to_str().unwrap()
    );

    let rd = fs::read_dir(directory);
    let list = List::Files { it: rd };
    self.entries.push(list);
  }
}

#[derive(Debug)]
enum List {
  Files { it: Result<ReadDir, io::Error> },
}

impl Iterator for List {
  type Item = Result<DirEntry, io::Error>;

  #[inline(always)]
  fn next(&mut self) -> Option<Result<DirEntry, io::Error>> {
    match *self {
      List::Files { ref mut it } => match *it {
        Err(ref err) => {
          warn!("error in list iterator: {}", err);
          return None;
        }
        Ok(ref mut rd) => rd.next(),
      },
    }
  }
}
