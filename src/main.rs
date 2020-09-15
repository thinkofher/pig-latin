use pig_latin::latin;

use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    for line in input.lock().lines() {
        println!("{}", latin::transcript(line.unwrap()));
    }
}
