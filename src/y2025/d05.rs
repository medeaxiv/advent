use std::{ops::RangeInclusive, str::FromStr};

use crate::{solution::Solution, util::invalid_input};

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(input: &str) -> anyhow::Result<(Ranges, Vec<u64>)> {
    let mut lines = input.lines();
    let ranges = (&mut lines)
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').ok_or_else(invalid_input!())?;
            let a = a.parse()?;
            let b = b.parse()?;
            Ok::<_, anyhow::Error>(a..=b)
        })
        .collect::<Result<_, _>>()?;
    let ranges = RangesBuilder(ranges).build();
    let ids = lines.map(FromStr::from_str).collect::<Result<_, _>>()?;
    Ok((ranges, ids))
}

fn parse_ranges(input: &str) -> anyhow::Result<Ranges> {
    let ranges = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').ok_or_else(invalid_input!())?;
            let a = a.parse()?;
            let b = b.parse()?;
            Ok::<_, anyhow::Error>(a..=b)
        })
        .collect::<Result<_, _>>()?;
    let ranges = RangesBuilder(ranges).build();
    Ok(ranges)
}

fn a(input: &str) -> anyhow::Result<usize> {
    let (ranges, ids) = parse(input)?;
    let count = ids.iter().filter(|id| ranges.contains(id)).count();
    Ok(count)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let ranges = parse_ranges(input)?;
    let count = ranges.iter().map(|r| r.len()).sum();
    Ok(count)
}

#[derive(Debug, Default, Clone)]
struct Ranges(Vec<RangeInclusive<u64>>);

impl Ranges {
    pub fn contains(&self, value: &u64) -> bool {
        for range in self.0.iter() {
            if range.start() > value {
                return false;
            }

            if range.contains(value) {
                return true;
            }
        }

        false
    }

    pub fn iter(&self) -> std::slice::Iter<'_, RangeInclusive<u64>> {
        self.0.iter()
    }
}

#[derive(Default, Clone)]
struct RangesBuilder(Vec<RangeInclusive<u64>>);

impl RangesBuilder {
    pub fn build(self) -> Ranges {
        let Self(mut ranges) = self;
        let ranges = Self::merge_ranges(&mut ranges);
        Ranges(ranges)
    }

    fn merge_ranges(ranges: &mut [RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
        if ranges.is_empty() {
            return Vec::new();
        }

        ranges.sort_unstable_by_key(|r| *r.start());
        // SAFETY: `ranges` is not empty. it has a first element.
        let mut merged = vec![ranges.first().unwrap().clone()];

        for range in &ranges[1..] {
            let start = *range.start();
            let end = *range.end();

            // SAFETY: `merged` is never empty
            let last = merged.last_mut().unwrap();
            if last.contains(&start) && !last.contains(&end) {
                let start = *last.start();
                *last = start..=end;
            } else if !last.contains(&start) {
                merged.push(start..=end);
            }
        }

        merged
    }
}

trait RangeExt<T> {
    fn len(&self) -> T;
}

impl<T> RangeExt<T> for RangeInclusive<T>
where
    T: Clone + num::Integer,
{
    fn len(&self) -> T {
        T::one() + (self.end().clone() - self.start().clone())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[rstest]
    #[case(TEST_INPUT, 3)]
    fn test_a(#[case] input: &str, #[case] expected: usize) {
        let result = super::a(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 14)]
    fn test_b(#[case] input: &str, #[case] expected: u64) {
        let result = super::b(input).unwrap();
        assert_eq!(result, expected);
    }
}
