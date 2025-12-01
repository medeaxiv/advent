use std::num::ParseIntError;

use num::traits::Euclid;

use crate::solution::Solution;

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(line: &str) -> Result<i32, ParseIntError> {
    let sign = if line.starts_with('L') { -1 } else { 1 };
    let magnitude: i32 = line[1..].parse()?;
    let value = sign * magnitude;
    Ok(value)
}

fn a(input: &str) -> anyhow::Result<i32> {
    let (_, password) = input
        .lines()
        .map(parse)
        .try_fold::<_, _, anyhow::Result<_>>((50, 0), |(position, password), amount| {
            let position = turn(position, amount?);
            let password = password + (position == 0) as i32;
            Ok((position, password))
        })?;

    Ok(password)
}

fn b(input: &str) -> anyhow::Result<i32> {
    let (_, password) = input
        .lines()
        .map(parse)
        .try_fold::<_, _, anyhow::Result<_>>((50, 0), |(position, password), amount| {
            let (position, zeros) = turn_count_zeros(position, amount?);
            let password = password + zeros;
            Ok((position, password))
        })?;

    Ok(password)
}

const DIAL: i32 = 100;

fn turn(position: i32, amount: i32) -> i32 {
    (position + amount).rem_euclid(DIAL)
}

fn turn_count_zeros(position: i32, amount: i32) -> (i32, i32) {
    let (mut zeros, end) = (position + amount).div_rem_euclid(&DIAL);
    zeros = zeros.abs();

    if amount < 0 {
        let was_zero = position == 0;
        let is_zero = end == 0;
        zeros -= was_zero as i32;
        zeros += is_zero as i32;
    }

    (end, zeros)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 0)]
    #[case(50, -68, 82)]
    #[case(82, -30, 52)]
    #[case(52, 48, 0)]
    #[case(0, -5, 95)]
    #[case(95, 60, 55)]
    #[case(55, -55, 0)]
    #[case(0, -1, 99)]
    #[case(99, -99, 0)]
    #[case(0, 14, 14)]
    #[case(14, -82, 32)]
    fn test_turn(#[case] position: i32, #[case] amount: i32, #[case] expected: i32) {
        let result = super::turn(position, amount);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(50, -68, 82)]
    #[case(82, -30, 52)]
    #[case(52, 48, 0)]
    #[case(0, -5, 95)]
    #[case(95, 60, 55)]
    #[case(55, -55, 0)]
    #[case(0, -1, 99)]
    #[case(99, -99, 0)]
    #[case(0, 14, 14)]
    #[case(14, -82, 32)]
    fn test_turn_count_zeros_position(
        #[case] position: i32,
        #[case] amount: i32,
        #[case] expected: i32,
    ) {
        let (result, _) = super::turn_count_zeros(position, amount);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(50, -68, 1)]
    #[case(82, -30, 0)]
    #[case(52, 48, 1)]
    #[case(0, -5, 0)]
    #[case(95, 60, 1)]
    #[case(55, -55, 1)]
    #[case(0, -1, 0)]
    #[case(99, -99, 1)]
    #[case(0, 14, 0)]
    #[case(14, -82, 1)]
    #[case(50, 1000, 10)]
    #[case(50, -1000, 10)]
    #[case(0, 1000, 10)]
    #[case(0, -1000, 10)]
    #[case(0, 100, 1)]
    #[case(0, -100, 1)]
    #[case(50, -150, 2)]
    fn test_turn_count_zeros_zeros(
        #[case] position: i32,
        #[case] amount: i32,
        #[case] expected: i32,
    ) {
        let (_, result) = super::turn_count_zeros(position, amount);
        assert_eq!(result, expected);
    }
}
