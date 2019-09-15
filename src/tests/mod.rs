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
  assert_eq!(finderResult.len(), 1);
}

#[test]
fn some_files() {
  //Arrange
  let tmp = TempDir::new("fonthelper_tests").unwrap();
  let collection = DataSet::new(&tmp.path()).some_files();

  //Act
  let finderResult: Vec<String> = Finder::new(&tmp.path())
    .into_iter()
    .map(|e| e.path().to_str().unwrap().to_string())
    .collect();

  //Assert
  assert_eq!(collection.len(), 8);
  assert_eq!(finderResult.len(), 8);
}

#[test]
fn one_depth() {
  //Arrange
  let tmp = TempDir::new("fonthelper_tests").unwrap();
  let collection = DataSet::new(&tmp.path()).one_depth();

  //Act
  let finderResult: Vec<String> = Finder::new(&tmp.path())
    .into_iter()
    .map(|e| e.path().to_str().unwrap().to_string())
    .collect();

  //Assert
  assert_eq!(collection.len(), 8);
  assert_eq!(finderResult.len(), 8);
}

#[test]
fn two_depth() {
  //Arrange
  let tmp = TempDir::new("fonthelper_tests").unwrap();
  let collection = DataSet::new(&tmp.path()).two_depth();

  //Act
  let finderResult: Vec<String> = Finder::new(&tmp.path())
    .into_iter()
    .map(|e| e.path().to_str().unwrap().to_string())
    .collect();

  //Assert
  assert_eq!(collection.len(), 10);
  assert_eq!(finderResult.len(), 10);
}

#[test]
fn five_depth() {
  //Arrange
  let tmp = TempDir::new("fonthelper_tests").unwrap();
  let collection = DataSet::new(&tmp.path()).five_depth();

  //Act
  let finderResult: Vec<String> = Finder::new(&tmp.path())
    .into_iter()
    .map(|e| e.path().to_str().unwrap().to_string())
    .collect();

  //Assert
  assert_eq!(collection.len(), 14);
  assert_eq!(finderResult.len(), 14);
}

#[test]
fn test_dataset_1() {
  //Arrange
  let tmp = TempDir::new("fonthelper_tests").unwrap();
  let collection = DataSet::new(&tmp.path()).dataset_1();

  //Act
  let finderResult: Vec<String> = Finder::new(&tmp.path())
    .into_iter()
    .map(|e| e.path().to_str().unwrap().to_string())
    .collect();

  //Assert
  assert_eq!(collection.len(), 800);
  assert_eq!(finderResult.len(), 800);
}
