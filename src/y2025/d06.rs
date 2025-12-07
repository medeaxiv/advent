use std::str::FromStr;

use crate::solution::Solution;

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn a(input: &str) -> anyhow::Result<u64> {
    let mut lines = input.lines();
    let operators: Vec<_> = lines
        .next_back()
        .ok_or_else(|| anyhow::anyhow!("empty input"))?
        .split_ascii_whitespace()
        .flat_map(Op::from_str)
        .collect();

    let mut results: Vec<_> = operators.iter().map(Op::identity).collect();
    for line in lines {
        let iter = line
            .split_ascii_whitespace()
            .zip(results.iter_mut().zip(operators.iter()));
        for (value, (result, op)) in iter {
            let value: u64 = value.parse()?;
            op.apply_to(result, value);
        }
    }

    let total = results.into_iter().sum();

    Ok(total)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let cols = 0..lines[0].len();
    let mut operands = Vec::new();
    let mut total = 0;
    for col in cols.rev() {
        let mut operand = 0;
        for line in lines.iter() {
            let ch = line[col] as char;
            match ch {
                '0'..='9' => {
                    let digit = ch as u64 - '0' as u64;
                    operand = (operand * 10) + digit;
                }
                '+' => {
                    operands.push(operand);
                    operand = 0;
                    total += operands.drain(..).sum::<u64>();
                }
                '*' => {
                    operands.push(operand);
                    operand = 0;
                    total += operands.drain(..).product::<u64>();
                }
                _ => continue,
            };
        }

        if operand != 0 {
            operands.push(operand);
        }
    }

    Ok(total)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    pub const fn identity(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }

    pub fn apply_to(&self, dest: &mut u64, value: u64) {
        match self {
            Self::Add => *dest += value,
            Self::Mul => *dest *= value,
        }
    }
}

impl FromStr for Op {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            _ => Ok(Self::Mul),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    pub const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[rstest]
    #[case(TEST_INPUT, 4277556)]
    fn test_a(#[case] input: &str, #[case] expected: u64) {
        let result = super::a(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 3263827)]
    fn test_b(#[case] input: &str, #[case] expected: u64) {
        let result = super::b(input).unwrap();
        assert_eq!(result, expected);
    }
}
