mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;

use crate::solution::Solutions;

pub fn register(solutions: &mut Solutions) {
    solutions.register(2025, 1, self::d01::solution());
    solutions.register(2025, 2, self::d02::solution());
    solutions.register(2025, 3, self::d03::solution());
    solutions.register(2025, 4, self::d04::solution());
    solutions.register(2025, 5, self::d05::solution());
    solutions.register(2025, 6, self::d06::solution());
    solutions.register(2025, 7, self::d07::solution());
    solutions.register(2025, 8, self::d08::solution());
    solutions.register(2025, 9, self::d09::solution());
}
