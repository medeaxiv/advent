use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{
    solution::Solution,
    util::{
        min_max,
        vector::{IVec2, vec2},
    },
};

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(input: &str) -> anyhow::Result<Vec<IVec2>> {
    let parser = |l: &str| {
        let (x, y) = l
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("invalid input"))?;
        let x = x.parse()?;
        let y = y.parse()?;
        Ok(vec2(x, y))
    };

    input.lines().map(parser).collect::<Result<_, _>>()
}

fn a(input: &str) -> anyhow::Result<i64> {
    let red_tiles = parse(input)?;
    let max = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rect::new(*a, *b).area())
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty input"))?;
    Ok(max)
}

fn b(input: &str) -> anyhow::Result<i64> {
    let red_tiles = parse(input)?;
    let segments = Segments::from_loop(&red_tiles)?;

    let mut max = None;
    for (a, b) in red_tiles.iter().tuple_combinations() {
        let rect = Rect::new(*a, *b);
        let Some(interior) = rect.interior() else {
            continue;
        };

        if segments.overlaps(&interior) {
            continue;
        }

        let area = rect.area();
        if let Some(max) = max.as_mut() {
            *max = area.max(*max);
        } else {
            max = Some(area);
        }
    }

    max.ok_or_else(|| anyhow::anyhow!("no suitable rectangle"))
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    position: IVec2,
    size: IVec2,
}

impl Rect {
    pub fn new(a: IVec2, b: IVec2) -> Self {
        let (ax, bx) = min_max(a.x, b.x);
        let (ay, by) = min_max(a.y, b.y);

        Self {
            position: vec2(ax, ay),
            size: vec2(bx - ax, by - ay) + vec2(1, 1),
        }
    }

    pub fn area(&self) -> i64 {
        self.size.x * self.size.y
    }

    pub fn interior(&self) -> Option<Self> {
        if self.size.x <= 1 || self.size.y <= 1 {
            None
        } else {
            Some(Self {
                position: self.position + vec2(1, 1),
                size: self.size - vec2(2, 2),
            })
        }
    }
}

struct Segments {
    horizontal: BTreeMap<i64, Vec<(i64, i64)>>,
    vertical: BTreeMap<i64, Vec<(i64, i64)>>,
}

impl Segments {
    pub fn from_loop(points: &[IVec2]) -> anyhow::Result<Self> {
        let mut horizontal: BTreeMap<_, Vec<_>> = Default::default();
        let mut vertical: BTreeMap<_, Vec<_>> = Default::default();
        for (a, b) in points.iter().circular_tuple_windows() {
            if a.y == b.y {
                horizontal.entry(a.y).or_default().push(min_max(a.x, b.x));
            } else if a.x == b.x {
                vertical.entry(a.x).or_default().push(min_max(a.y, b.y));
            } else {
                return Err(anyhow::anyhow!("diagonal segment"));
            }
        }

        Ok(Self {
            horizontal,
            vertical,
        })
    }

    pub fn overlaps(&self, rect: &Rect) -> bool {
        let x = (rect.position.x, rect.position.x + rect.size.x);
        let y = (rect.position.y, rect.position.y + rect.size.y);

        Self::overlaps_inner(&self.horizontal, &y, &x).is_some()
            || Self::overlaps_inner(&self.vertical, &x, &y).is_some()
    }

    fn overlaps_inner(
        segments: &BTreeMap<i64, Vec<(i64, i64)>>,
        across: &(i64, i64),
        along: &(i64, i64),
    ) -> Option<(i64, i64, i64)> {
        for (at, ranges) in segments.range(across.0..across.1) {
            for range in ranges.iter() {
                if along.0 < range.1 && range.0 < along.1 {
                    return Some((*at, range.0, range.1));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::util::vector::{IVec2, vec2};

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[rstest]
    #[case(vec2(2, 5), vec2(9, 7), 24)]
    #[case(vec2(7, 1), vec2(11, 7), 35)]
    #[case(vec2(7, 3), vec2(2, 3), 6)]
    #[case(vec2(2, 5), vec2(11, 1), 50)]
    fn test_area(#[case] a: IVec2, #[case] b: IVec2, #[case] expected: i64) {
        let result = super::Rect::new(a, b).area();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 50)]
    fn test_a(#[case] input: &str, #[case] expected: i64) {
        let result = super::a(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 24)]
    fn test_b(#[case] input: &str, #[case] expected: i64) {
        let result = super::b(input).unwrap();
        assert_eq!(result, expected);
    }
}
