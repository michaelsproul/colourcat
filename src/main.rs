extern crate ansi_term;
extern crate regex;
extern crate rand;

mod colour;

use std::io::{self, BufRead};
use std::collections::HashMap;
use ansi_term::Colour;
use regex::{Regex, Captures};
use colour::choose_colour;

fn main() {
    let pattern = Regex::new(r"((\d|[a-f]){6}…(\d|[a-f]){6}|\d{3,})").unwrap();
    let stdin = io::stdin();

    let mut colours = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let result = pattern.replace_all(&line, |cap: &Captures| {
            let item = &cap[0];

            let prev_colours = colours.clone(); // eurgh, slow.
            let &mut (r, g, b) = colours.entry(item.to_string()).or_insert_with(|| {
                choose_colour(&prev_colours)
            });
            format!("{}", Colour::RGB(r, g, b).paint(item))
        });
        println!("{}", result);
    }

    /*
    for (string, colour) in colours {
        let luminance = match colour {
            RGB(r, g, b) => luminance(r, g, b),
            _ => 0.0
        };
        println!("{}: {}", colour.paint(string), luminance);
    }
    */
}
