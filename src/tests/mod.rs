#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
mod utils;

#[cfg(test)]
use super::Finder;
#[cfg(test)]
use tempdir::TempDir;
#[cfg(test)]
use utils::DataSet;

#[test]
fn test_random_dataset() {
  for _ in 0..10 {
    //Arrange
    let tmp = TempDir::new("fonthelper_tests").unwrap();
    let collection = DataSet::new(&tmp.path()).gen();

    //Act
    let finder_result: Vec<String> = Finder::new(&tmp.path())
      .into_iter()
      .map(|e| e.path().to_str().unwrap().to_string())
      .collect();

    //Assert
    assert_eq!(collection.len(), finder_result.len());
  }
}
