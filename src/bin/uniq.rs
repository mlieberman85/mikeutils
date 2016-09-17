use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let first = lines.next();
    
    let mut previous = match first.unwrap() { 
        Ok(some_string) => some_string,
        Err(e) => process::exit(1)
    };

    println!("{}", previous);

    for line in lines {
        let cursor = line.unwrap();
        if cursor != previous {
            println!("{}", cursor);
        }
        previous = cursor;
    }
}
