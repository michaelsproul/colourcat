extern crate ansi_term;
extern crate regex;
extern crate rand;

use std::io::{self, BufRead};
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use ansi_term::Colour;
use regex::{Regex, Captures};

const MIN_LUMINANCE: f64 = 90.0;
const MIN_DISTANCE: f64 = 10.0;

type RGB = (u8, u8, u8);

fn colour_dist((ir1, ig1, ib1): (u8, u8, u8), (ir2, ig2, ib2): (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = (ir1 as f64, ig1 as f64, ib1 as f64);
    let (r2, g2, b2) = (ir2 as f64, ig2 as f64, ib2 as f64);
    (0.299 * (r1 - r2).powi(2) + 0.587 * (g1 - g2).powi(2) + 0.114 * (b1 - b2).powi(2)).sqrt()
}

fn luminance(red: u8, green: u8, blue: u8) -> f64 {
    colour_dist((red, green, blue), (0, 0, 0))
}

fn choose_colour(colours: &HashMap<String, RGB>) -> RGB {
    'search: loop {
        let (r1, g1, b1) = thread_rng().gen();
        if luminance(r1, g1, b1) < MIN_LUMINANCE {
            continue;
        }
        for &(r2, g2, b2) in colours.values() {
            if colour_dist((r1, g1, b1), (r2, g2, b2)) < MIN_DISTANCE {
                continue 'search;
            }
        }
        return (r1, g1, b1);
    }
}

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
