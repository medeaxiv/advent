mod d01;

use crate::solution::Solutions;

pub fn register(solutions: &mut Solutions) {
    solutions.register(2025, 1, self::d01::solution())
}
