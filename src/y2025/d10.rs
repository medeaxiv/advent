use z3::{Optimize, SatResult, ast::Int};

use crate::solution::Solution;

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn a(input: &str) -> anyhow::Result<u64> {
    let mut total = 0;
    for machine in input.lines().map(parser::parse) {
        let machine = machine?;
        let (_pattern, presses) = machine.solve_lights();
        total += presses;
    }

    Ok(total)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let mut total = 0;
    for machine in input.lines().map(parser::parse) {
        let machine = machine?;
        total += machine.solve_joltage();
    }

    Ok(total)
}

fn iter_ones(bits: u32) -> impl Iterator<Item = u32> {
    (0..32).filter(move |&b| (1 << b) & bits != 0)
}

#[derive(Debug, Clone)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}

impl Machine {
    pub fn new(lights: u32, buttons: Vec<u32>, joltage: Vec<u32>) -> Self {
        Self {
            lights,
            buttons,
            joltage,
        }
    }

    pub fn solve_lights(&self) -> (u32, u64) {
        let limit = 1 << self.buttons.len();
        let mut min = limit;
        let mut min_pattern = 0;
        for pattern in 0..limit {
            let result =
                iter_ones(pattern).fold(0, |lights, button| lights ^ self.buttons[button as usize]);

            let presses = pattern.count_ones();
            if result == self.lights && presses < min {
                min = presses;
                min_pattern = pattern;
            }
        }

        (min_pattern, min as u64)
    }

    pub fn solve_joltage(&self) -> u64 {
        let opt = Optimize::new();
        let mut buttons = Vec::with_capacity(self.buttons.len());
        let mut terms: Vec<Vec<Int>> = vec![vec![]; self.joltage.len()];
        for (i, &button) in self.buttons.iter().enumerate() {
            let btn = Int::fresh_const(&format!("btn{i}"));

            for e in iter_ones(button) {
                terms[e as usize].push(btn.clone());
            }

            opt.assert(&btn.ge(0));
            buttons.push(btn);
        }

        let total = Int::fresh_const("total");
        opt.assert(&total.eq(Int::add(&buttons)));
        for (terms, &target) in terms.iter().zip(self.joltage.iter()) {
            let sum = Int::add(terms.as_slice());
            opt.assert(&sum.eq(target));
        }

        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => opt
                .get_model()
                .expect("check found a solution, there is a model")
                .eval(&total, true)
                .expect("total is part of the model")
                .as_u64()
                .expect("total is a sum of positive integers"),
            _ => unreachable!(),
        }
    }
}

mod parser {
    use nom::{
        IResult, Parser,
        bytes::{complete::tag, take_until},
        character::complete::u32,
        multi::{fold_many0, many1, separated_list1},
        sequence::{delimited, preceded},
    };

    use crate::util::invalid_input;

    use super::Machine;

    type Error<'i> = nom::error::Error<&'i str>;

    fn final_parse<I, O, E: nom::error::ParseError<I>>(
        mut parser: impl Parser<I, Output = O, Error = E>,
        input: I,
    ) -> Result<O, E> {
        use nom::Err;
        match parser.parse(input) {
            Ok((_, out)) => Ok(out),
            Err(Err::Error(e)) => Err(e),
            Err(Err::Failure(e)) => Err(e),
            Err(Err::Incomplete(_)) => unreachable!(),
        }
    }

    pub fn parse(line: &str) -> anyhow::Result<Machine> {
        let lights = delimited(tag("["), take_until("]"), tag("] ")).map(parse_lights);

        let button = delimited(tag("("), parse_button, tag(") "));
        let buttons = many1(button);

        let joltage = separated_list1(tag(","), u32);
        let joltage = delimited(tag("{"), joltage, tag("}"));

        let parser = (lights, buttons, joltage).map(|(l, b, j)| Machine::new(l, b, j));

        final_parse::<_, _, Error>(parser, line).map_err(invalid_input!(e))
    }

    fn parse_lights(s: &str) -> u32 {
        s.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .fold(0, |a, (i, _)| a | (1 << i))
    }

    fn parse_button<'i, I, E>(input: I) -> IResult<I, u32, E>
    where
        I: nom::Input + nom::Compare<&'i str>,
        I::Item: nom::AsChar,
        E: nom::error::ParseError<I>,
    {
        let mut bit = u32.map(|v| 1 << v);
        let (input, button) = bit.parse(input)?;
        let rest = preceded(tag(","), bit);
        let result = fold_many0(rest, || button, |a, b| a | b).parse(input)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUTS: &[&str] = &[
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];

    #[rstest]
    #[case(TEST_INPUTS[0], 2)]
    #[case(TEST_INPUTS[1], 3)]
    #[case(TEST_INPUTS[2], 2)]
    fn test_a(#[case] input: &str, #[case] expected: u64) {
        let machine = super::parser::parse(input).unwrap();
        let (_pattern, presses) = machine.solve_lights();
        assert_eq!(presses, expected);
    }

    #[rstest]
    #[case(TEST_INPUTS[0], 10)]
    #[case(TEST_INPUTS[1], 12)]
    #[case(TEST_INPUTS[2], 11)]
    fn test_b(#[case] input: &str, #[case] expected: u64) {
        let machine = super::parser::parse(input).unwrap();
        let result = machine.solve_joltage();
        assert_eq!(result, expected);
    }
}
