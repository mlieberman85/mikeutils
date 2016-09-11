use std::io;
use std::io::prelude::*;
use std::fs;
use std::path::Path;
use std::env;

fn print_dir(name: &Path, realname: &str) {
    let dir_iterator = fs::read_dir(name);
    for entry in dir_iterator.unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("\x1b[0;31m{}\x1b[0;31m", path.display());
        }
        else {
            println!("{}", path.display());
        }
    }

}

fn main() {
    let stdin = io::stdin();

    let current_path = env::current_dir().unwrap();
    let path = current_path.as_path();
    print_dir(path, "");
}


