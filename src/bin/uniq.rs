use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut previous = None;

    for line in stdin.lock().lines() {
        let cursor = line.unwrap();
        if let Some(x) = previous {
            if cursor != x {
                println!("{}", cursor);
            }
        } else {
            println!("{}", cursor);
        }
        previous = Some(cursor);
    }
}
