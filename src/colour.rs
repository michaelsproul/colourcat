use metrohash::MetroHash64;
use std::hash::Hasher;

const MIN_LUMINANCE: f64 = 100.0;

pub type RGB = (u8, u8, u8);

pub fn colour_dist((ir1, ig1, ib1): (u8, u8, u8), (ir2, ig2, ib2): (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = (ir1 as f64, ig1 as f64, ib1 as f64);
    let (r2, g2, b2) = (ir2 as f64, ig2 as f64, ib2 as f64);
    (0.299 * (r1 - r2).powi(2) + 0.587 * (g1 - g2).powi(2) + 0.114 * (b1 - b2).powi(2)).sqrt()
}

pub fn luminance(red: u8, green: u8, blue: u8) -> f64 {
    colour_dist((red, green, blue), (0, 0, 0))
}

pub fn hash_colour(string: &str) -> RGB {
    let mut hasher = MetroHash64::new();
    hasher.write(string.as_bytes());
    let hash = hasher.finish();

    let (mut red, mut green, mut blue) = (hash as u8, (hash >> 8) as u8, (hash >> 16) as u8);

    let mut i = 24;
    let k = 1;

    while luminance(red, green, blue) < MIN_LUMINANCE {
        let rand_value = (hash >> i) & 0b11;
        match rand_value {
            0 => {
                red += k;
            }
            1 => {
                green += k;
            }
            2 => {
                blue += k;
            }
            _ => {
                red += 1;
                green += 1;
                blue += 1;
            }
        }
        i = (i + 1) % 64;
    }

    (red, green, blue)
}
