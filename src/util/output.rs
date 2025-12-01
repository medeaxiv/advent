use std::{borrow::Cow, fmt::Display};

use num::{
    BigInt, BigUint,
    complex::{Complex32, Complex64},
    rational::{BigRational, Rational32, Rational64},
};

pub trait Output: Display {
    fn is_multiline(&self) -> bool;
}

macro_rules! impl_single_line_output {
    ($ty:ty, $( $tail:ty ),* $(,)?) => {
        impl_single_line_output!($ty);
        impl_single_line_output!($( $tail ),*);
    };
    ($ty:ty) => {
        impl Output for $ty {
            fn is_multiline(&self) -> bool {
                false
            }
        }
    };
}

impl_single_line_output! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    f32, f64,
    Rational32, Rational64,
    BigInt, BigUint, BigRational,
    Complex32, Complex64,
}

macro_rules! impl_str_output {
    ($ty:ty, $( $tail:ty ),* $(,)?) => {
        impl_str_output!($ty);
        impl_str_output!($( $tail ),*);
    };
    ($ty:ty) => {
        impl Output for $ty {
            fn is_multiline(&self) -> bool {
                self.contains(&['\n', '\r'])
            }
        }
    }
}

impl_str_output! {
    &str,
    String,
    Cow<'_, str>,
}
