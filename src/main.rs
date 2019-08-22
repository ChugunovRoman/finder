extern crate finder;

use std::fs::DirEntry;

use finder::finder::Finder;

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
    let finders = Finder::new("/usr/share/fonts/opentype:/usr/share/fonts/truetype/noto");
    for i in finders.filter(&is_font_file).into_iter() {
        println!("{}", i.path().to_str().expect("FAIL!!!!"));
    }
}
