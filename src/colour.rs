use std::collections::HashMap;
use rand::{thread_rng, Rng};

const MIN_LUMINANCE: f64 = 90.0;
const MIN_DISTANCE: f64 = 10.0;
const MAX_ITERATIONS: usize = 50;

pub type RGB = (u8, u8, u8);

pub fn colour_dist((ir1, ig1, ib1): (u8, u8, u8), (ir2, ig2, ib2): (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = (ir1 as f64, ig1 as f64, ib1 as f64);
    let (r2, g2, b2) = (ir2 as f64, ig2 as f64, ib2 as f64);
    (0.299 * (r1 - r2).powi(2) + 0.587 * (g1 - g2).powi(2) + 0.114 * (b1 - b2).powi(2)).sqrt()
}

pub fn luminance(red: u8, green: u8, blue: u8) -> f64 {
    colour_dist((red, green, blue), (0, 0, 0))
}

pub fn choose_colour(colours: &HashMap<String, RGB>) -> RGB {
    'search: for _ in 0..MAX_ITERATIONS {
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
    thread_rng().gen()
}

