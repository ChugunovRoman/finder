
use std::fmt;
use std::io;
use std::fs::{self, ReadDir, DirEntry};
use std::path::{Path, PathBuf};

struct FinderOptions {
    regex: String,
}

impl fmt::Debug for FinderOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("FinderOptions")
            .field("regex", &self.regex)
            .finish()
    }
}

pub struct Finder {
    opts: FinderOptions,
    root: PathBuf,
}

impl Finder {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Finder {
            opts: FinderOptions {
                regex: String::from("*")
            },
            root: root.as_ref().to_path_buf()
        }
    }

    pub fn regex(mut self, regex: String) -> Self {
        self.opts.regex = regex;
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
                self.handle_entry(&start);
            }
        }

        while !self.entries.is_empty() {
            let next = self.entries
                .last_mut()
                .expect("BUG: dirs should be non-empty")
                .next();
            match next {
                None => self.entries.pop(),
                Some(entry) => {
                    let e = entry.expect("BUGG");
                    self.handle_entry(&e.path());
                    if !e.path().is_dir() {
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
                Err(ref mut err) => None,
                Ok(ref mut rd) => rd.next()
            }
        }
    }
}
