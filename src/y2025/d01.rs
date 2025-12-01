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
            let (position, _) = turn(position, amount?);
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
            let (position, zeros) = turn(position, amount?);
            let password = password + zeros;
            Ok((position, password))
        })?;

    Ok(password)
}

const DIAL: i32 = 100;

fn turn(position: i32, amount: i32) -> (i32, i32) {
    let (zeros, end) = (position + amount).div_rem_euclid(&DIAL);
    let mut zeros = zeros.abs();

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
    #[case(0, 0, (0, 0))]
    #[case(50, -68, (82, 1))]
    #[case(82, -30, (52, 0))]
    #[case(52, 48, (0, 1))]
    #[case(0, -5, (95, 0))]
    #[case(95, 60, (55, 1))]
    #[case(55, -55, (0, 1))]
    #[case(0, -1, (99, 0))]
    #[case(99, -99, (0, 1))]
    #[case(0, 14, (14, 0))]
    #[case(14, -82, (32, 1))]
    #[case(50, 1000, (50, 10))]
    #[case(50, -1000, (50, 10))]
    #[case(0, 1000, (0, 10))]
    #[case(0, -1000, (0, 10))]
    #[case(0, 100, (0, 1))]
    #[case(0, -100, (0, 1))]
    #[case(50, 150, (0, 2))]
    #[case(50, -150, (0, 2))]
    fn test_turn(#[case] position: i32, #[case] amount: i32, #[case] expected: (i32, i32)) {
        let result = super::turn(position, amount);
        assert_eq!(result, expected);
    }
}
