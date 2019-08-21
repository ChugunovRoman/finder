extern crate regex;

mod filder;

use std::{fs, io};
use std::path::Path;

use regex::Regex;

// fn read_dir(directory: &str) -> Result<(), io::Error> {
//     let dir = Path::new(directory);

//     if !dir.is_dir() {
//         println!("{} is not directory!", directory);
//         return Ok(());
//     }

//     for entry in fs::read_dir(dir).expect("Cannor read dir") {
//         let entry = entry.expect("");
//         println!("path: {} {}", entry.path().display(), entry.path().is_dir());
//     }

//     Ok(())
// }

fn main() {
    let reg = Regex::new(r"\.tt(f|c)").unwrap();

    // read_dir("/home/ruut").unwrap();

    let finders = filder::Finder::new("/usr/share/fonts");
    for i in finders.into_iter() {
        println!("FILE: {}", i.path().to_str().expect("FAIL!!!!"));
    }
    // for i in filder::finder().take(9) {
    //     println!("entry: {}", i);
    // }

    // let output = std::process::Command::new("find")
    //     .arg("/usr/share/fonts")
    //     .arg("-type")
    //     .arg("f")
    //     .output()
    //     .expect("find cmd failed");

    // println!("output: {}", String::from_utf8(output.stdout).unwrap());
}
