#![allow(dead_code)]

pub mod bitmap;
pub mod char;
pub mod grid;
pub mod output;
pub mod style;
pub mod vecset;

pub fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if b < a { (b, a) } else { (a, b) }
}
