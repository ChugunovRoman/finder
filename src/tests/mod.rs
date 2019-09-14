#[cfg(test)]
extern crate tempdir;

mod utils;

use super::Finder;
#[cfg(test)]
use tempdir::TempDir;
use utils::DataSet;

#[test]
fn empty() {
  //Arrange
  let tmp = TempDir::new("fonthelper_tests").unwrap();
  let collection = DataSet::new(&tmp.path()).empty_dir();

  //Act
  let finderResult: Vec<String> = Finder::new(&tmp.path())
    .into_iter()
    .map(|e| e.path().to_str().unwrap().to_string())
    .collect();

  //Assert
  assert_eq!(collection.len(), 0);

  println!("Test empty directory");
  assert_eq!(finderResult.len(), 0);
}

#[test]
fn one_file() {
  //Arrange
  let tmp = TempDir::new("fonthelper_tests").unwrap();
  let collection = DataSet::new(&tmp.path()).one_file();

  //Act
  let finderResult: Vec<String> = Finder::new(&tmp.path())
    .into_iter()
    .map(|e| e.path().to_str().unwrap().to_string())
    .collect();

  //Assert
  assert_eq!(collection.len(), 1);

  println!("Test one file in directory");
  assert_eq!(finderResult.len(), 1);
}
