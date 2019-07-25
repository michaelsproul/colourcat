mod colour;

use ansi_term::Colour;
use colour::hash_colour;
use regex::{Captures, Regex};
use std::io::{self, BufRead};

fn main() {
    // TODO: make the regex adjustable from the CLI
    let pattern = Regex::new(r"((0x)?(\d|[a-fA-F]){6}(…|\.)?(\d|[a-fA-F]){6,}|\d{3,})").unwrap();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let result = pattern.replace_all(&line, |cap: &Captures| {
            let item = &cap[0];
            let (r, g, b) = hash_colour(item);
            format!("{}", Colour::RGB(r, g, b).paint(item))
        });
        println!("{}", result);
    }
}
