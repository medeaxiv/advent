use foldhash::{HashMap, HashSet};

use crate::solution::Solution;

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

#[derive(Debug, Default, Clone)]
struct Input {
    width: i64,
    height: i64,
    start: i64,
    splitters: HashSet<(i64, i64)>,
}

fn parse(input: &str) -> Input {
    let mut parsed = Input::default();
    for (y, line) in input.lines().step_by(2).enumerate() {
        parsed.height += 1;
        for (x, ch) in line.chars().enumerate() {
            if y == 0 {
                parsed.width += 1;
            }

            match ch {
                'S' => {
                    parsed.start = x as i64;
                }
                '^' => {
                    parsed.splitters.insert((x as i64, y as i64));
                }
                _ => {}
            }
        }
    }

    parsed
}

fn a(input: &str) -> anyhow::Result<u64> {
    let input = parse(input);

    let mut beams = HashMap::from_iter([(input.start, 1)]);
    let mut buffer = beams.clone();
    let mut splits = 0;
    for y in 1..input.height {
        with_split_beams(&input.splitters, y, &beams, |x, count| {
            splits += 1;
            buffer.remove(&x);
            buffer.entry(x - 1).and_modify(add(count)).or_insert(count);
            buffer.entry(x + 1).and_modify(add(count)).or_insert(count);
        });

        beams = buffer.clone();
    }

    Ok(splits)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let input = parse(input);

    let mut beams = HashMap::from_iter([(input.start, 1)]);
    let mut buffer = beams.clone();
    for y in 1..input.height {
        with_split_beams(&input.splitters, y, &beams, |x, count| {
            buffer.remove(&x);
            buffer.entry(x - 1).and_modify(add(count)).or_insert(count);
            buffer.entry(x + 1).and_modify(add(count)).or_insert(count);
        });

        beams = buffer.clone();
    }

    let timelines = beams.values().copied().sum();
    Ok(timelines)
}

fn with_split_beams<F>(splitters: &HashSet<(i64, i64)>, y: i64, beams: &HashMap<i64, u64>, mut f: F)
where
    F: FnMut(i64, u64),
{
    for (&x, &count) in beams.iter() {
        let key = (x, y);
        if splitters.contains(&key) {
            f(x, count);
        }
    }
}

fn add<T>(value: T) -> impl Fn(&mut T)
where
    T: std::ops::AddAssign + Copy,
{
    move |dest| *dest += value
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[rstest]
    #[case(TEST_INPUT, 21)]
    fn test_a(#[case] input: &str, #[case] expected: u64) {
        let result = super::a(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 40)]
    fn test_b(#[case] input: &str, #[case] expected: u64) {
        let result = super::b(input).unwrap();
        assert_eq!(result, expected);
    }
}
