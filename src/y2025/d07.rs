use std::num::NonZeroU64;

use foldhash::HashSet;

use crate::{solution::Solution, util::slice::SliceExt};

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

    let mut beams = vec![None; input.width as usize];
    beams[input.start as usize] = NonZeroU64::new(1);
    let mut buffer = beams.clone();
    let mut splits = 0;
    for y in 1..input.height {
        with_split_beams(&input.splitters, y, &beams, |x, count| {
            splits += 1;
            let x = x as usize;
            buffer[x] = None;
            let [left, right] = buffer
                .multi_index_mut([x - 1, x + 1])
                .expect("indices should be in bounds and different");
            add(left, count);
            add(right, count);
        });

        beams = buffer.clone();
    }

    Ok(splits)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let input = parse(input);

    let mut beams = vec![None; input.width as usize];
    beams[input.start as usize] = NonZeroU64::new(1);
    let mut buffer = beams.clone();
    for y in 1..input.height {
        with_split_beams(&input.splitters, y, &beams, |x, count| {
            let x = x as usize;
            buffer[x] = None;
            let [left, right] = buffer
                .multi_index_mut([x - 1, x + 1])
                .expect("indices should be in bounds and different");
            add(left, count);
            add(right, count);
        });

        beams = buffer.clone();
    }

    let timelines = beams.iter().flat_map(|c| c.map(NonZeroU64::get)).sum();
    Ok(timelines)
}

fn with_split_beams<F>(
    splitters: &HashSet<(i64, i64)>,
    y: i64,
    beams: &[Option<NonZeroU64>],
    mut f: F,
) where
    F: FnMut(i64, u64),
{
    for (x, count) in beams
        .iter()
        .enumerate()
        .flat_map(|(index, count)| count.map(|count| (index as i64, count.get())))
    {
        let key = (x, y);
        if splitters.contains(&key) {
            f(x, count);
        }
    }
}

fn add(dest: &mut Option<NonZeroU64>, count: u64) {
    if let Some(dest) = dest.as_mut() {
        *dest = (*dest).saturating_add(count);
    } else {
        *dest = NonZeroU64::new(count);
    }
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
