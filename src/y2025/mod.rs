mod d01;
mod d02;
mod d03;

use crate::solution::Solutions;

pub fn register(solutions: &mut Solutions) {
    solutions.register(2025, 1, self::d01::solution());
    solutions.register(2025, 2, self::d02::solution());
    solutions.register(2025, 3, self::d03::solution());
}
