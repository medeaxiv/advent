mod d01;
mod d02;

use crate::solution::Solutions;

pub fn register(solutions: &mut Solutions) {
    solutions.register(2025, 1, self::d01::solution());
    solutions.register(2025, 2, self::d02::solution());
}
