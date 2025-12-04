use foldhash::HashSet;

use crate::solution::Solution;

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut rolls = HashSet::default();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((x as i32, y as i32));
            }
        }
    }

    rolls
}

fn a(input: &str) -> anyhow::Result<u64> {
    let rolls = parse(input);

    let mut accessible = 0;
    with_accessible_rolls(&rolls, |_, _| accessible += 1);
    Ok(accessible)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let mut rolls = parse(input);
    let initial = rolls.len();
    let mut removed = Vec::new();
    let mut dirty = HashSet::default();
    with_accessible_rolls(&rolls, |x, y| removed.push((x, y)));

    while !removed.is_empty() {
        for roll in removed.drain(..) {
            dirty.extend(moore_neighborhood(roll.0, roll.1));
            rolls.remove(&roll);
        }

        with_accessible_rolls_masked(&rolls, &dirty, |x, y| removed.push((x, y)));
        dirty.clear();
    }

    let remaining = rolls.len();
    let removed = initial - remaining;
    Ok(removed as u64)
}

fn with_accessible_rolls<F>(rolls: &HashSet<(i32, i32)>, mut f: F)
where
    F: FnMut(i32, i32),
{
    for &(x, y) in rolls {
        let neighborhood = moore_neighborhood(x, y);
        let rolls = neighborhood.filter(|n| rolls.contains(n));
        let roll_count = rolls.count();

        if roll_count < 4 {
            f(x, y)
        }
    }
}

fn with_accessible_rolls_masked<F>(
    rolls: &HashSet<(i32, i32)>,
    mask: &HashSet<(i32, i32)>,
    mut f: F,
) where
    F: FnMut(i32, i32),
{
    for &(x, y) in rolls.intersection(mask) {
        let neighborhood = moore_neighborhood(x, y);
        let rolls = neighborhood.filter(|n| rolls.contains(n));
        let roll_count = rolls.count();

        if roll_count < 4 {
            f(x, y)
        }
    }
}

const MOORE_NEIGHBORHOOD: &[(i32, i32); 8] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn moore_neighborhood(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    MOORE_NEIGHBORHOOD
        .iter()
        .map(move |&(xoff, yoff)| (x + xoff, y + yoff))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[rstest]
    #[case(TEST_INPUT)]
    fn test_a(#[case] input: &str) {
        let result = super::a(input).unwrap();
        assert_eq!(result, 13);
    }

    #[rstest]
    #[case(TEST_INPUT)]
    fn test_b(#[case] input: &str) {
        let result = super::b(input).unwrap();
        assert_eq!(result, 43);
    }
}
