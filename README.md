# Finder

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/02e634f24d7a47f581110dd6ead027aa)](https://app.codacy.com/app/ChugunovRoman/finder?utm_source=github.com&utm_medium=referral&utm_content=ChugunovRoman/finder&utm_campaign=Badge_Grade_Dashboard)
<span class="badge-buymeacoffee"><a href="https://www.buymeacoffee.com/U5hnMuASy" title="Donate to this project using Buy Me A Coffee"><img src="https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg" alt="Buy Me A Coffee donate button" /></a></span>
<span class="badge-paypal"><a href="https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=4DNBUKPV6FBCY&source=url" title="Donate to this project using Paypal"><img src="https://img.shields.io/badge/paypal-donate-yellow.svg" alt="PayPal donate button" /></a></span>

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
