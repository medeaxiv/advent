use std::fmt::{Display, Write};

use crate::util::{grid::Grid, output::Output};

pub trait AsBit {
    fn as_bit(&self) -> bool;
}

impl AsBit for bool {
    fn as_bit(&self) -> bool {
        *self
    }
}

pub struct Bitmap<T>(pub Grid<T>);

impl<T> Display for Bitmap<T>
where
    T: AsBit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BitmapDisplay(&self.0))
    }
}

impl<T> Output for Bitmap<T>
where
    T: AsBit,
{
    fn is_multiline(&self) -> bool {
        true
    }
}

pub struct BitmapDisplay<'g, T>(pub &'g Grid<T>);

impl<T> BitmapDisplay<'_, T>
where
    T: AsBit,
{
    fn bit(&self, x: u32, y: u32) -> bool {
        self.0.get(x, y).is_some_and(T::as_bit)
    }

    fn braille(&self, x: u32, y: u32) -> char {
        let x0 = x * 2;
        let x1 = x0 + 1;
        let y0 = y * 4;
        let y1 = y0 + 1;
        let y2 = y0 + 2;
        let y3 = y0 + 3;

        braille_char(
            self.bit(x0, y0),
            self.bit(x1, y0),
            self.bit(x0, y1),
            self.bit(x1, y1),
            self.bit(x0, y2),
            self.bit(x1, y2),
            self.bit(x0, y3),
            self.bit(x1, y3),
        )
    }
}

impl<T> Display for BitmapDisplay<'_, T>
where
    T: AsBit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = braille_width(self.0.width());
        let height = braille_height(self.0.height());

        for y in 0..height {
            if y != 0 {
                writeln!(f)?;
            }

            for x in 0..width {
                let ch = self.braille(x, y);
                f.write_char(ch)?;
            }
        }
        Ok(())
    }
}

fn braille_width(width: u32) -> u32 {
    let rem = width % 2;
    let width = width / 2;
    width + ((rem != 0) as u32)
}

fn braille_height(height: u32) -> u32 {
    let rem = height % 4;
    let height = height / 4;
    height + ((rem != 0) as u32)
}

#[inline(always)]
const fn bb(bit: bool, shift: u32) -> u32 {
    (bit as u32) << shift
}

#[allow(clippy::too_many_arguments)]
fn braille_char(
    d00: bool,
    d01: bool,
    d10: bool,
    d11: bool,
    d20: bool,
    d21: bool,
    d30: bool,
    d31: bool,
) -> char {
    const BLOCK: u32 = 0x2800;

    let dots_bits = bb(d00, 0)
        | bb(d10, 1)
        | bb(d20, 2)
        | bb(d01, 3)
        | bb(d11, 4)
        | bb(d21, 5)
        | bb(d30, 6)
        | bb(d31, 7);
    let codepoint = BLOCK | dots_bits;

    // SAFETY: since `dots_bits` only occuppies the first 8 bits, `codepoint` always falls within
    // the Braille Patterns unicode block
    unsafe { char::from_u32_unchecked(codepoint) }
}
