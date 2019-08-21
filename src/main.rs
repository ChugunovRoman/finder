extern crate finder;

use finder::finder::Finder;

fn main() {
    let finders = Finder::new("/usr/share/fonts");
    for i in finders.into_iter() {
        println!("{}", i.path().to_str().expect("FAIL!!!!"));
    }
}
