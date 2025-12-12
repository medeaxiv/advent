#![allow(dead_code)]

pub mod bitmap;
pub mod char;
pub mod grid;
pub mod output;
pub mod slice;
pub mod style;
pub mod vecset;
pub mod vector;
pub mod write;

pub fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if b < a { (b, a) } else { (a, b) }
}

macro_rules! invalid_input {
    () => {
        || ::anyhow::anyhow!("invalid input")
    };
    ($reason:literal) => {
        || ::anyhow::anyhow!(concat!("invalid input: ", $reason))
    };
    ($reasion:literal, $($e:expr),* $(,)?) => {
        || ::anyhow::anyhow!(concat!("invalid input: ", $reason), $($expr),*)
    };
}

pub(crate) use invalid_input;
