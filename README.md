# Finder

Crate `finder` is a very simple and lightweight file searcher with the filtering of files.
It provides an efficient implementation of recursive file search.

To use this crate, add `finder` as a dependency to your project's
`Cargo.toml`:

```toml
[dependencies]
finder = "0.1"
```

## Example

The following code recursively search all files in `/foo` and `/bar` diresctories:

```rust
extern crate finder;

use finder::Finder;

fn main() {
  let finders = Finder::new("/foo:/bar");
  for i in finders.into_iter() {
    println!("{}", i.path().to_str().unwrap());
  }
}
```

## Example with filter

The following code recursively search `.ttf` and `.ttc` files in `/foo` and `/bar` diresctories:

```rust
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
